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
//use link::link_helpers::*;
use score::*;
// use std::str;
use std::thread;
// use std::u16;
// use transport::socket::*;


/// In memory representation of a Logical Link Control header (SNAP variant only).
pub struct LlcHeader
{
	pub ether_type: u16,
}

// See
// https://en.wikipedia.org/wiki/IEEE_802.2
// https://en.wikipedia.org/wiki/Subnetwork_Access_Protocol
// https://tools.ietf.org/html/rfc1042
impl LlcHeader
{
	pub fn with_ipv4() -> Self
	{
		LlcHeader {ether_type: 0x0800}
	}

	/// Adds an LLC header to the packet.
	pub fn push(&self, packet: &mut Packet)
	{
		let mut header = Header::with_capacity(8);

		// llc header
		let dsap = 170;			// SNAP dst
		header.push8(dsap);

		let ssap = 170;			// SNAP src
		header.push8(ssap);

		let control = 3;		// connectionless
		header.push8(control);

		// snap extension
		let oui = 0;			// protocol id is an EtherType
		header.push8(oui);
		header.push8(oui);
		header.push8(oui);

		let protocol_id = self.ether_type;
		header.push16(protocol_id);

		packet.push_header(&header);
	}

	/// Removes an LLC header from the packet.
	pub fn pop(packet: &mut Packet) -> Result<LlcHeader, String>
	{
		let dsap = packet.pop8();
		if dsap != 170 {
			return Err("DSAP isn't 170".to_string())
		}

		let ssap = packet.pop8();
		if ssap != 170 {
			return Err("SSAP isn't 170".to_string())
		}

		let control = packet.pop8();
		if control != 3 {
			return Err("CONTROL isn't 3".to_string())
		}

		let oui0 = packet.pop8();
		let oui1 = packet.pop8();
		let oui2 = packet.pop8();
		if oui0 != 0 || oui1 != 0 || oui2 != 0 {
			return Err("OUI isn't 0".to_string())
		}

		Ok(LlcHeader {ether_type: packet.pop16()})
	}
}

/// Component that pushes and pops a Logical Link Control header.
pub struct LlcComponent
{
	data: ThreadData,

	/// Listens for "send_down" events.
	pub upper_in: InPort<(IPv4Header, Packet)>,	
	pub upper_out: OutPort<(LinkInfo, Packet)>,

	/// Listens for "send_up" events.
	pub lower_in: InPort<(MacAddress, MacAddress, Packet)>,
	pub lower_out: OutPort<(MacAddress, MacAddress, Packet)>,
}

impl LlcComponent
{
	pub fn new(sim: &mut Simulation, parent_id: ComponentID) -> Self
	{
		let (id, data) = sim.add_active_component("LLC", parent_id);
		LlcComponent {
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
					let (ipv4, mut packet) = event.take_payload::<(IPv4Header, Packet)>();
					let header = LlcHeader::with_ipv4();
					header.push(&mut packet);

               		let src_addr = [0, 0, ipv4.src_addr[0], ipv4.src_addr[1], ipv4.src_addr[2], ipv4.src_addr[3]];	// TODO: need to use an ARP table
               		let dst_addr = [0, 0, ipv4.dst_addr[0], ipv4.dst_addr[1], ipv4.dst_addr[2], ipv4.dst_addr[3]];
					self.lower_out.send_payload(&mut effector, &event.name, (src_addr, dst_addr, packet));
				},
				"send_up" => {
					let (src_addr, dst_addr, mut packet) = event.take_payload::<(MacAddress, MacAddress, Packet)>();
					match LlcHeader::pop(&mut packet) {
						Ok(header) => {
							let linfo = LinkInfo::new(header.ether_type, &src_addr, &dst_addr);
							self.upper_out.send_payload(&mut effector, &event.name, (linfo, packet));
						},
						Err(mesg) => log_warning!(effector, "pop failed: {}", mesg)
					}
				}
			);
		});
	}
}
