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
use common::*;
// use internet::*;
// use link::link::*;
// use link::link_helpers::*;
use score::*;
use std::fs;
use std::io;
use std::io::Write;
use std::thread;

/// Component used to write our pcap files for use by tools such as Wireshark or tshark.
/// There are several component settings that affect how this is done:
/// - **count** is the maximum number of packets to write out. Defaults to 100_000.
/// - **path** is the file in which the pcap is stored. If this is empty then no pcap is generated. Defaults to empty.
/// - **promiscuous** if 1 save all frames that were seen. Otherwise only save frames addressed to this device. Defaults to 0. TODO: not implemented
/// - **snap_length** is the maximum number of packet bytes to write out. Defaults to 65536.
/// 
pub struct PcapComponent	
{
	data: ThreadData,

	/// Listens for "send_down" events.
	pub upper_in: InPort<Packet>,	
	pub upper_out: OutPort<Packet>,

	/// Listens for "send_up" events.
	pub lower_in: InPort<Packet>,
	pub lower_out: OutPort<Packet>,
}

// TODO: add direction setting? tcpdump uses in, out, and inout for the values
// TODO: add support for radiotap headers?
impl PcapComponent
{
	pub fn new(sim: &mut Simulation, parent_id: ComponentID) -> Self
	{
		let (id, data) = sim.add_active_component("pcap", parent_id);
		PcapComponent {
			data: data,

			upper_in: InPort::with_port_name(id, "upper_in"),
			lower_out: OutPort::new(),

			lower_in: InPort::with_port_name(id, "lower_in"),
			upper_out: OutPort::new(),
		}
	}
	
	pub fn start(self)
	{
		let mut file: io::Result<fs::File>  = Err(io::Error::new(io::ErrorKind::Other, "no path"));

		thread::spawn(move || {
			process_events!(self.data, event, state, effector,
				"init 0" => {
					if !state.contains(self.data.id, "count") {
						effector.set_int("count", 100_000);	// we really only need to do all this if we have a path but we always do it so that our values look nicer in GUIs
					}
					if !state.contains(self.data.id, "promiscuous") {
						effector.set_int("promiscuous", 0);
					}
					if !state.contains(self.data.id, "snap_length") {
						effector.set_int("snap_length", 65536);
					}
					if !state.contains(self.data.id, "path") {
						log_excessive!(effector, "not saving pcap");
						effector.set_string("path", "");
					} else {
						let path = state.get_string(self.data.id, "path");	// TODO: document how to initialize this
						file = fs::File::create(&path);
						if file.is_ok() {
							log_info!(effector, "saving pcap to {}", path)
						} else {
							log_error!(effector, "failed to create pcap: {:?}", file)
						}
					}
					effector.set_int("frame", 0);
				},
				"send_down" => {
					let packet = event.take_payload::<Packet>();
					if let Ok(ref mut f) = file {
						self.record_frame(f, &mut effector, &state, &packet);
					}
					self.lower_out.send_payload(&mut effector, "send_up", packet);
				},
				"send_up" => {
					let packet = event.take_payload::<Packet>();
					if let Ok(ref mut f) = file {
						self.record_frame(f, &mut effector, &state, &packet);
					}

					self.upper_out.send_payload(&mut effector, &event.name, packet);
				}
			);
		});
	}

	fn record_frame(&self, file: &mut fs::File, effector: &mut Effector, state: &SimState, packet: &Packet)
	{
		let frame = state.get_int(self.data.id, "frame") + 1;
		let count = state.get_int(self.data.id, "count");
		if frame <= count {
			let result = write!(file, "frame {}: {:?}\n", frame, packet);	// TODO: check result, maybe close handle if there were errors
			effector.set_int("frame", frame);
		}
	}
}
