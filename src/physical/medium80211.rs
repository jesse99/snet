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
//use internet::*;
// use internet::protocol_numbers::*;
// use internet::upper_internet::*;
// use link::link::*;
// use link::link_helpers::*;
use score::*;
// use std::str;
use std::thread;
// use std::u16;
// use transport::socket::*;

// Unless otherwise indicated all references are to the 2016 version of "Part 11: Wireless LAN Medium Access Control (MAC) and Physical Layer (PHY) Specifications"
// (i.e. the 802.11 spec).

/// Wireless medium for 802.11 wireless radios.
pub struct Medium80211Component
{
	data: ThreadData,

	pub upper_ins: Vec<InPort<(ComponentID, Packet)>>,
	pub upper_outs: Vec<OutPort<Packet>>,
}

impl Medium80211Component
{
	pub fn new(sim: &mut Simulation, parent_id: ComponentID) -> Self
	{
		let (_, data) = sim.add_active_component("Medium80211", parent_id);
		Medium80211Component {data: data, upper_ins: Vec::new(), upper_outs: Vec::new()}
	}
	
	pub fn connect(&mut self, above_out: &mut OutPort<(ComponentID, Packet)>, above_in: &InPort<Packet>)
	{
		let upper_in = InPort::with_port_name(self.data.id, &format!("upper_in_{}", self.upper_ins.len()));
		let mut upper_out = OutPort::new();

		above_out.connect_to(&upper_in);
		upper_out.connect_to(&above_in);

		self.upper_ins.push(upper_in);
		self.upper_outs.push(upper_out);
	}
	
	pub fn start(self)
	{		
		thread::spawn(move || {
			process_events!(self.data, event, state, effector,
				"init 0" => {
				},
				"send_down" => {
					assert!(!event.port_name.is_empty());

					let (_, packet) = event.take_payload::<(ComponentID, Packet)>();	// TODO: from_id should be used to compute bit errors and whether the frame is below the noise floor
					for i in 0..self.upper_outs.len() {
						if self.upper_ins[i].target_port != event.port_name {
							let port = &self.upper_outs[i];
							port.send_payload(&mut effector, "send_up", packet.clone());
						}
					}
				}
			);
		});
	}
}
