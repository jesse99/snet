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
// extern crate rand;
#[macro_use]
extern crate score;
extern crate snet;

use clap::{App, ArgMatches};
//use rand::{Rng, SeedableRng, StdRng};
use score::*;
use snet::*;
use std::fmt::Display;
use std::process;
use std::str::FromStr;
//use std::thread;

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

fn handle_sender(app: &user::AppComponent, event: &Event, state: &SimState, effector: &mut Effector)
{
	match event.name.as_ref() {
		"init 0" => {
			log_info!(effector, "init");
		
			let info = internet::InternetInfo::new(internet::UDP, common::IPAddress::IPv4([10, 0, 0, 1]), common::IPAddress::IPv4([127, 0, 0, 2]));
			let options = transport::SocketOptions::with_addr(common::IPAddress::IPv4([127, 0, 0, 2]));
			let mut packet = common::Packet::new("packet", "#>1");
			let payload = "hello".to_string();
			packet.push_bytes(payload.as_bytes());
			app.upper_out.send_payload_after_secs(effector, "send_down", 1.0, (info, options, packet));

			//let event = Event::new("timer");
			//effector.schedule_immediately(event, self.id);
		},		
		_ => {
			let cname = &(*state.components).get(app.data.id).name;
			panic!("component {} can't handle event {}", cname, event.name);
		}
	}
}

fn handle_receiver(app: &user::AppComponent, event: &Event, state: &SimState, effector: &mut Effector)
{
	match event.name.as_ref() {
		"init 0" => {
			log_info!(effector, "init");
		
			//let event = Event::new("timer");
			//effector.schedule_immediately(event, self.id);
		},		
		"send_up" => {
			log_info!(effector, "received a packet!");
		},		
		_ => {
			let cname = &(*state.components).get(app.data.id).name;
			panic!("component {} can't handle event {}", cname, event.name);
		}
	}
}

fn create_sim(local: LocalConfig, config: Config) -> Simulation
{
	let mut sim = Simulation::new(config);
	let world_id = sim.add_component("world", NO_COMPONENT);

	let mut sender = devices::Endpoint::new("sender", &mut sim, world_id);
	let mut receiver = devices::Endpoint::new("receiver", &mut sim, world_id);
	sender.app.callback = Some(handle_sender);
	receiver.app.callback = Some(handle_receiver);
	sender.connect(&mut receiver);
		
	// This is used by GUIs, e.g. sdebug.
	let mut effector = Effector::new();
	effector.set_float("display-size-x", DISPLAY_WIDTH);
	effector.set_float("display-size-y", DISPLAY_HEIGHT);
	effector.set_string("display-title", "echo");
	sim.apply(world_id, effector);

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
