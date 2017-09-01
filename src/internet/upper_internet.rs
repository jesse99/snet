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
use internet::ipv4::*;
use internet::internet::*;
// use internet::protocol_numbers::*;
use score::*;
// use std::str;
use std::thread;
use transport::socket::*;

/// Handles dispatching downward packets to either [`IPv4Component`] or [`IPv6Component`].
/// For upward packets this will (normally) send an event to [`LowerTransportComponent`].
pub struct UpperInternetComponent
{
	data: ThreadData,

	/// Listens for "send_down" events.
	pub upper_in: InPort<(InternetInfo, SocketOptions, Packet)>,
	pub upper_out: OutPort<(InternetInfo, Packet)>,
	// TODO: IPv6

	/// Listens for "send_up" events.
	pub lower_in: InPort<(InternetInfo, Packet)>,
	pub lower_ipv4_out: OutPort<(IPv4Header, Packet)>,
}

impl UpperInternetComponent
{
	pub fn new(sim: &mut Simulation, parent_id: ComponentID) -> UpperInternetComponent
	{
		let (id, data) = sim.add_active_component("IPv4", parent_id);
		UpperInternetComponent {
			data: data,

			upper_in: InPort::with_port_name(id, "upper_in"),
			lower_ipv4_out: OutPort::new(),

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
					let (iinfo, options, packet) = event.take_payload::<(InternetInfo, SocketOptions, Packet)>();
					let header = IPv4Header::with_internet(&iinfo, &options);
					self.lower_ipv4_out.send_payload(&mut effector, &event.name, (header, packet));
				},
				"send_up" => {
					let (info, packet) = event.take_payload::<(InternetInfo, Packet)>();
					self.upper_out.send_payload(&mut effector, &event.name, (info, packet));
				}
			);
		});
	}
}
