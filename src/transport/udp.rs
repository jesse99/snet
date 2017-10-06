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
//use internet::protocol_numbers::*;
use score::*;
// use std::str;
use std::thread;
use std::u16;
use transport::*;

/// In memory version of the UDP header. When serialized to a [`Packet`] it's packed according to the spec.
pub struct UDPHeader
{
	/// Usually an ephemeral or well-known port.
	pub src_port: u16,
	
	/// An ephemeral or well-known port.
	pub dst_port: u16,
}

// See https://en.wikipedia.org/wiki/User_Datagram_Protocol#Packet_structure
impl UDPHeader
{
	pub fn new(src_port: u16, dst_port: u16) -> Self
	{	
		UDPHeader {
			src_port,
			dst_port,
		}
	}

	/// Adds a UDP header to the packet.
	pub fn push(&self, packet: &mut Packet, info: &InternetInfo)
	{
		let payload_len = packet.len();
		let total_len = 8 + payload_len;
		assert!(total_len < u16::MAX as usize);

		let mut header = Header::with_capacity(8);
		header.push16(self.src_port);
		header.push16(self.dst_port);
		header.push16(total_len as u16);
		header.push16(0);	// TODO: checksum is optional for IPv4 but mandatory for IPv6 (and we need to compute it using an annoying pseudo-header)

		let crc1 = UDPHeader::psuedo_header_checksum(packet, info);
		let crc2 = header.start_checksum(crc1);
		let crc = packet.finish_checksum(packet.len(), crc2);
		header.data[6] = (crc >> 8) as u8;
		header.data[7] = (crc & 0xFF) as u8;	

		packet.push_header(&header);
	}

	/// Removes a UDP header from the packet.
	pub fn pop(packet: &mut Packet) -> Result<UDPHeader, String>
	{
		let in_len = packet.len();

		let src_port = packet.pop16();
		let dst_port = packet.pop16();
		let total_length = packet.pop16() as usize;
		let _ = packet.pop16();

		if total_length != in_len {
			return Err(format!("UDPHeader.total_length should be {} but is {}", in_len, total_length))
		}

		let header = UDPHeader {src_port, dst_port};
		Ok(header)
	}

	fn psuedo_header_checksum(packet: &Packet, info: &InternetInfo) -> u32
	{
		match info.dst_addr {
			IPAddress::IPv4(dst_addr) => {
				match info.src_addr {
					IPAddress::IPv4(src_addr) => {
						let mut header = Header::with_capacity(12);
						header.push8(src_addr[0]);		// source IP
						header.push8(src_addr[1]);
						header.push8(src_addr[2]);
						header.push8(src_addr[3]);

						header.push8(dst_addr[0]);		// destination IP
						header.push8(dst_addr[1]);
						header.push8(dst_addr[2]);
						header.push8(dst_addr[3]);

						header.push8(0);				// zeros
						header.push8(17);				// protocol

						let len = packet.len() as u16;
						header.push16(8 + len);			// header len + data len

						header.start_checksum(0)
					},
					IPAddress::IPv6(_) => panic!("InternetInfo has mixed IPv4 and IPv6 addresses")
				}
			},
			IPAddress::IPv6(_) => panic!("IPv6 isn't supported yet")
		}
	}
}

/// Pushes an UDPHeader onto packets moving down the network stack.
/// Pops off an UDPHeader header for packets moving up the stack.
pub struct UdpComponent
{
	data: ThreadData,

	/// Listens for "send_down" events.
	pub upper_in: InPort<(InternetInfo, SocketOptions, Packet)>,	// TODO: need a port from somewhere, take a Socket?
	pub upper_out: OutPort<(InternetInfo, Packet)>,	

	/// Listens for "send_up" events.
	pub lower_in: InPort<(InternetInfo, Packet)>,
	pub lower_out: OutPort<(InternetInfo, SocketOptions, Packet)>,
}

impl UdpComponent
{
	pub fn new(sim: &mut Simulation, parent_id: ComponentID) -> Self
	{
		let (id, data) = sim.add_active_component("UDP", parent_id);
		UdpComponent {
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
					let (info, options, mut packet) = event.take_payload::<(InternetInfo, SocketOptions, Packet)>();
					let src_port = 1;	// TODO: use an epheremal port
					let header = UDPHeader::new(src_port, 19);
					header.push(&mut packet, &info);

					self.lower_out.send_payload(&mut effector, &event.name, (info, options, packet));
				},
				"send_up" => {
					let (info, mut packet) = event.take_payload::<(InternetInfo, Packet)>();
					match UDPHeader::pop(&mut packet) {
						Ok(_header) => self.upper_out.send_payload(&mut effector, &event.name, (info, packet)),
						Err(mesg) => log_warning!(effector, "pop failed: {}", mesg)
					}
				}
			);
		});
	}
}
