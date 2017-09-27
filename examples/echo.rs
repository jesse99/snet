// Copyright (C) 2017 Jesse Jones
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3, or (at your option)
// any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software Foundation,
// Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.

//! This example sends a packet to a remote endpoint which then sends it back.
#[macro_use]
extern crate clap;
#[macro_use]
extern crate score;
extern crate snet;

use clap::{App, ArgMatches};
use score::*;
use snet::*;
use std::fmt::Display;
use std::process;
use std::str;
use std::str::FromStr;
use std::thread;

const DISPLAY_WIDTH: f64 = 50.0;
const DISPLAY_HEIGHT: f64 = 100.0;

#[derive(Clone)]
struct LocalConfig
{
	packets: i32,
}

impl LocalConfig
{
	fn new() -> LocalConfig
	{
		// These are the defaults: all of them can be overriden using command line options.
		LocalConfig {
			packets: 1,
		}
	}
}

fn rx_packet(event: &mut Event, effector: &mut Effector, expected_payload: &str)
{
	let (_, mut packet) = event.take_payload::<(internet::InternetInfo, common::Packet)>();
	let len = packet.len();
	let data = packet.pop_bytes(len);
	match str::from_utf8(data.as_slice()) {
		Ok(text) if text == expected_payload => log_info!(effector, "received a packet!"),
		Ok(text) => log_error!(effector, "received a bad packet: '{}'", text),
		Err(mesg) => log_error!(effector, "received a bad packet: {}", mesg)
	}
}

fn handle_sender(app: &user::AppComponent, event: &mut Event, state: &SimState, effector: &mut Effector)
{
	match event.name.as_ref() {
		"init 0" => {		
			let event = Event::new("timer");
			effector.schedule_after_secs(event, app.data.id, 1.0);
		},		
		"timer" => {		
			let info = internet::InternetInfo::new(internet::UDP, common::IPAddress::IPv4([10, 0, 0, 1]), common::IPAddress::IPv4([127, 0, 0, 2]));
			let options = transport::SocketOptions::with_addr(common::IPAddress::IPv4([127, 0, 0, 2]));
			let mut packet = common::Packet::new("packet", "#>1");
			let payload = "hello".to_string();
			packet.push_back_bytes(payload.as_bytes());
			app.upper_out.send_payload_after_secs(effector, "send_down", 1.0, (info, options, packet));

			let sent = if state.contains(app.data.id, "num_sent") {state.get_int(app.data.id, "num_sent")} else {0};
			effector.set_int("num_sent", sent+1);
		},		
		"send_up" => {
			rx_packet(event, effector, "echoed hello");

			let recv = if state.contains(app.data.id, "num_recv") {state.get_int(app.data.id, "num_recv")} else {0};
			effector.set_int("num_recv", recv+1);
		
			let to_send = state.store.get_int("world.packets");
			if recv+1 == to_send {
				let event = Event::new("finished");
				let (world_id, _) = state.components.get_root();
				effector.schedule_immediately(event, world_id);
			} else {
				let event = Event::new("timer");
				effector.schedule_after_secs(event, app.data.id, 1.0);
			}
		},		
		_ => {
			let cname = &(*state.components).get(app.data.id).name;
			panic!("component {} can't handle event {}", cname, event.name);
		}
	}
}

fn handle_receiver(app: &user::AppComponent, event: &mut Event, state: &SimState, effector: &mut Effector)
{
	match event.name.as_ref() {
		"init 0" => {
			log_info!(effector, "init");
		},		
		"send_up" => {
			rx_packet(event, effector, "hello");

			let count = if state.contains(app.data.id, "num_recv") {state.get_int(app.data.id, "num_recv")} else {0};
			effector.set_int("num_recv", count+1);
		
			let info = internet::InternetInfo::new(internet::UDP, common::IPAddress::IPv4([10, 0, 0, 2]), common::IPAddress::IPv4([127, 0, 0, 1]));
			let options = transport::SocketOptions::with_addr(common::IPAddress::IPv4([127, 0, 0, 1]));
			let mut packet = common::Packet::new("packet", "#>2");
			let payload = "echoed hello".to_string();
			packet.push_back_bytes(payload.as_bytes());
			app.upper_out.send_payload_after_secs(effector, "send_down", 1.0, (info, options, packet));
		},		
		_ => {
			let cname = &(*state.components).get(app.data.id).name;
			panic!("component {} can't handle event {}", cname, event.name);
		}
	}
}

fn world_thread(local: &LocalConfig, data: ThreadData)
{
	fn check_eq(effector: &mut Effector, store: &Store, path: &str, expected: i64)
	{
		if store.contains(path) {
			let actual = store.get_int(path);
			if actual == expected {
				let message = format!("{} is {}", path, actual);
				log_info!(effector, &message);
			} else {
				let message = format!("{} is {} but {} was expected", path, actual, expected);
				log_error!(effector, &message);
			}
		} else {
			let message = format!("{} is missing (expected {})", path, expected);
			log_error!(effector, &message);
		}
	}

	let packets = local.packets as i64;
	thread::spawn(move || {
		process_events!(data, event, state, effector,
			"init 0" => {
			},
			"finished" => {
				check_eq(&mut effector, &state.store, "world.receiver.app.num_recv", packets);
				check_eq(&mut effector, &state.store, "world.sender.app.num_recv", packets);
			}
		);
	});
}

fn create_sim(local: LocalConfig, config: Config) -> Simulation
{
	let mut sim = Simulation::new(config);
	let (world_id, world_data) = sim.add_active_component("world", NO_COMPONENT);
	world_thread(&local, world_data);

	let mut sender = devices::Endpoint::new("sender", &mut sim, world_id);
	let mut receiver = devices::Endpoint::new("receiver", &mut sim, world_id);
	sender.app.callback = Some(handle_sender);
	receiver.app.callback = Some(handle_receiver);
	sender.connect(&mut receiver);
		
	sim.configure(|id, component, components, effector| {
		match component.name.as_ref() {
			"world" => {
				// This is used by GUIs, e.g. sdebug.
				effector.set_int("packets", local.packets as i64);
				effector.set_float("display-size-x", DISPLAY_WIDTH);
				effector.set_float("display-size-y", DISPLAY_HEIGHT);
				effector.set_string("display-title", "echo");
			},
			"pcap" => {
				// Save off a pcap file for each device.
				let (_, top) = components.get_top(id);
				effector.set_string("path", &(top.name.clone() + ".pcap"));
			},
			_ => {}
		}
	});

	// and spin up their threads.
	sender.start(&mut sim);
	receiver.start(&mut sim);
	
	sim
}

fn fatal_err(message: &str) -> !
{
	eprintln!("{}", message);
	process::exit(1);
}

// Min and max are inclusive.
fn match_num<T>(matches: &ArgMatches, name: &str, min: T, max: T) -> T
		where T: Copy + Display + FromStr + PartialOrd
{
	match value_t!(matches.value_of(name), T) {
		Ok(value) if value < min => fatal_err(&format!("--{} should be greater than {}", name, min)),
		Ok(value) if value > max => fatal_err(&format!("--{} should be less than {}", name, max)),
		Ok(value) => value,
		_ => fatal_err(&format!("--{} should be a number", name)),
	}
}

fn parse_options() -> (LocalConfig, Config)
{
	let mut local = LocalConfig::new();
	let mut config = Config::new();
	
	// see https://docs.rs/clap/2.24.2/clap/struct.Arg.html#method.from_usage for syntax
	let usage = format!(	// TODO: would be nice to avoid this duplication (match_num and fatal_err too)
		"--address=[ADDR] 'Address for the web server to bind to [{default_address}]'
		--home=[PATH] 'Start the web server and serve up PATH when / is hit'
		--log=[LEVEL:GLOB]... 'Overrides --log-level, glob is used to match component names'
		--log-level=[LEVEL] 'Default log level: {log_levels} [{default_level}]'
		--max-time=[TIME] 'Maximum time to run the simulation, use {time_suffixes} suffixes [no limit]'
		--no-colors 'Don't color code console output'
		--packets=[N] 'Number of steps between the sender and receiver [{default_packets}]'
		--seed=[N] 'Random number generator seed [random]'",
		default_address = config.address,
		default_packets = local.packets,
		default_level = format!("{:?}", config.log_level).to_lowercase(),
		log_levels = log_levels(),
		time_suffixes = time_suffixes());
	
	let matches = App::new("echo")
		.version("1.0")
		.author("Jesse Jones <jesse9jones@gmail.com>")
		.about("Send a packet and replt.")
		.args_from_usage(&usage)
	.get_matches();
		
	if matches.is_present("packets") {
		local.packets = match_num(&matches, "packets", 1, 10_000);
	}
	
	if matches.is_present("seed") {
		config.seed = match_num(&matches, "seed", 1, usize::max_value());
	}
	
	if matches.is_present("address") {
		config.address = matches.value_of("address").unwrap().to_string();
	}
	
	if matches.is_present("home") {
		config.home_path = matches.value_of("home").unwrap().to_string();
	}
	
	if matches.is_present("log-level") {
		if let Some(e) = config.parse_log_level(matches.value_of("log-level").unwrap()) {
			fatal_err(&e);
		}
	}

	if matches.is_present("log") {
		if let Some(e) = config.parse_log_levels(matches.values_of("log").unwrap().collect()) {
			fatal_err(&e);
		}
	}
	
	let max_secs = matches.value_of("max-time").unwrap_or("");
	if !max_secs.is_empty() {
		if let Some(e) = config.parse_max_secs(max_secs) {
			fatal_err(&e);
		}
	}
	
	config.colorize = !matches.is_present("no-colors");
	
	(local, config)
}

fn main()
{
	let (local, mut config) = parse_options();
	config.time_units = 1_000_000.0;	// us resolution
	
	let mut sim = create_sim(local, config);
	sim.run();
}
