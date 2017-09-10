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
use internet::*;
// use internet::protocol_numbers::*;
// use internet::upper_internet::*;
use link::link::*;
use score::*;
// use std::str;
use std::thread;
// use std::u16;
// use transport::socket::*;

/// MAC that relies on a perfectly ideal wire.
pub struct IdealMacComponent	// TODO: get rid of this later?
{
	data: ThreadData,

	/// Listens for "send_down" events.
	pub upper_in: InPort<(IPv4Header, Packet)>,	
	pub upper_out: OutPort<(LinkInfo, Packet)>,

	/// Listens for "send_up" events.
	pub lower_in: InPort<(Packet)>,
	pub lower_out: OutPort<(Packet)>,
}

impl IdealMacComponent
{
	pub fn new(sim: &mut Simulation, parent_id: ComponentID) -> Self
	{
		let (id, data) = sim.add_active_component("IdealMac", parent_id);
		IdealMacComponent {
			data: data,

			upper_in: InPort::with_port_name(id, "upper_in"),
			lower_out: OutPort::new(),

			lower_in: InPort::with_port_name(id, "lower_in"),
			upper_out: OutPort::new(),
		}
	}
	
	pub fn start(self)
	{		
		thread::spawn(move || {
			process_events!(self.data, event, state, effector,
				"init 0" => {
				},
				"send_down" => {
					let (_, packet) = event.take_payload::<(IPv4Header, Packet)>();
					self.lower_out.send_payload(&mut effector, "send_up", packet);
				},
				"send_up" => {
					let packet = event.take_payload::<Packet>();
					let linfo = LinkInfo::new(0, 0, 0);	// TODO: need to push and pop this
					self.upper_out.send_payload(&mut effector, &event.name, (linfo, packet));
				}
			);
		});
	}
}
