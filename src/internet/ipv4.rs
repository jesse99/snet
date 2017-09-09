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
use internet::internet::*;
use internet::protocol_numbers::*;
//use internet::upper_internet::*;
use link::*;
use score::*;
use std::str;
use std::thread;
use std::u16;
use transport::*;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum ECN
{
	NotCapable = 0,
	Capable0 = 1,
	Capable1 = 2,
	Congested = 3,
}

/// In memory version of the IPv4 header. When serialized to a [`Packet`] it's packed according to the spec.
pub struct IPv4Header
{
	/// TCP, UDP, IGMP, OSPF, etc.
	pub protocol: u8,
	
	/// The sender of the packet.
	pub src_addr: [u8; 4],
	
	/// The destination of the packet.
	pub dst_addr: [u8; 4],

	/// Differentiated services field (formerly TOS). Defined by RFC 2474 with updates from
	/// RFC 3168 and RFC 3260.
	pub dscp: u8,
	
	/// Explicit congestion notification field. Used by TCP (when both endpoints support it).
	pub ecn: ECN,
	
	/// Time to live: maximum number of hops the packet is allowed to travel.
	pub ttl: u8,

	// If the packet is too large then drop it instead of fragmenting it.
	pub dont_fragment: bool,

	// Set for fragmented packets (except for the very last fragment).
	pub more_fragments: bool,
	
	/// The offset (in bytes) of the fragment within the original packet.
	pub fragment_offset: u16,
	
	/// Used to re-assemble fragmented packets.
	pub identification: u16,
}

// See https://en.wikipedia.org/wiki/IPv4#Packet_structure
impl IPv4Header
{
	pub fn new(protocol: u8, src_addr: [u8; 4], dst_addr: [u8; 4], options: &SocketOptions) -> IPv4Header
	{	
		assert!(protocol != RESERVED);

		IPv4Header {
			protocol,
			src_addr,
			dst_addr,
			dscp: options.priority,
			ecn: ECN::NotCapable,	// TODO: TCP should set this, I guess with SocketOptions
			ttl: options.ttl,
			dont_fragment: options.dont_fragment,
			more_fragments: false,	
			fragment_offset: 0,
			identification: 0,
		}
	}

	pub fn with_internet(info: &InternetInfo, options: &SocketOptions) -> IPv4Header
	{	
		match info.dst_addr {
			IPAddress::IPv4(dst_addr) => {
				match info.src_addr {
					IPAddress::IPv4(src_addr) => {
						IPv4Header {
							protocol: info.protocol,
							src_addr,
							dst_addr,
							dscp: options.priority,
							ecn: ECN::NotCapable,	// TODO: TCP should set this, I guess with SocketOptions
							ttl: options.ttl,
							dont_fragment: options.dont_fragment,
							more_fragments: false,
							fragment_offset: 0,
							identification: 0,
						}
					},
					IPAddress::IPv6(_) => panic!("InternetInfo has mixed IPv4 and IPv6 addresses")
				}
			},
			IPAddress::IPv6(_) => panic!("IPv6 isn't supported yet")
		}
	}

	/// Adds an IP header to the packet.
	pub fn push(&self, packet: &mut Packet)
	{
		let payload_len = packet.len();
		let mut header = Header::with_capacity(20);

		let b = 0x45;						// version + IHL (we don't support options so length is fixed)
		header.push8(b);

		assert!(self.dscp < 64);			// dscp + ecn
		let b = self.dscp << 2 | (self.ecn as u8);
		header.push8(b);

		let hw = 20 + payload_len;			// total length
		assert!(hw <= u16::MAX as usize);
		header.push16(hw as u16);

		header.push16(self.identification);	// identification

		assert!(self.fragment_offset < 8192);	// flags + frag offset
		let hw = (self.more_fragments as u16) << 15 | (self.dont_fragment as u16) << 14 | self.fragment_offset;
		//println!("pushed {}", hw);
		header.push16(hw);
	
		header.push8(self.ttl);				// ttl
		header.push8(self.protocol);		// protocol

		let hw = 0;							// checksum (this is set for real after we've pushed the header)
		header.push16(hw);

		header.push8(self.src_addr[0]);	// source IP
		header.push8(self.src_addr[1]);
		header.push8(self.src_addr[2]);
		header.push8(self.src_addr[3]);

		header.push8(self.dst_addr[0]);	// destination IP
		header.push8(self.dst_addr[1]);
		header.push8(self.dst_addr[2]);
		header.push8(self.dst_addr[3]);

		let crc = header.checksum();
		header.data[10] = (crc >> 8) as u8;
		header.data[11] = (crc & 0xFF) as u8;	
		//println!("header = {:?}", header);

		packet.push_header(&header);
	}

	/// Removes an IP header from the packet.
	pub fn pop(packet: &mut Packet) -> Result<IPv4Header, String>
	{
		let in_len = packet.len();
		let crc = packet.checksum(20);
		if crc != 0 {
			return Err("IPv4Header checksum error".to_string())
		}

		let b = packet.pop8();
		let version = b >> 4;
		let ihl = b & 0xF;
		if version != 4 {				// packets can be corrupted and CRC checks won't always catch it so we need to verify that the header is still legit
			return Err(format!("IPv4Header.version should be 4 not {}", version))
		}
		if ihl != 5 {
			return Err(format!("IPv4Header.IHL should be 5 not {}", ihl))
		}

		let b = packet.pop8();
		let dscp = b >> 2;
		let ecn = match b & 0x3 {
			0 => ECN::NotCapable,
			1 => ECN::Capable0,
			2 => ECN::Capable1,
			3 => ECN::Congested,
			_ => panic!("should never get a value larger than 3 from 2 bits")
		};

		let total_length = packet.pop16() as usize;
		if total_length != in_len {
			return Err(format!("IPv4Header.total_length should be {} but is {}", in_len, total_length))
		}

		let identification = packet.pop16();

		let hw = packet.pop16();
		let more_fragments = hw & 0x8000 != 0;
		let dont_fragment = hw & 0x4000 != 0;
		let reserved = hw & 0x2000 != 0;
		let fragment_offset = hw & 0x1FFF;
		if reserved {
			return Err(format!("IPv4Header.flags has bit 0 set"))
		}

		let ttl = packet.pop8();
		let protocol = packet.pop8();
		if protocol == RESERVED {
			return Err(format!("IPv4Header.protocol is using the RESERVED protocol (use one of the unassigned values instead for a custom protocol)"))
		}

		let _ = packet.pop16();		// this is the checksum (which we actually checked first thing)

		let src_addr = [packet.pop8(), packet.pop8(), packet.pop8(), packet.pop8()];
		let dst_addr = [packet.pop8(), packet.pop8(), packet.pop8(), packet.pop8()];
	
		let header = IPv4Header {
			protocol,
			src_addr,
			dst_addr,
			dscp,
			ecn,
			ttl,
			identification,
			dont_fragment,
			more_fragments,
			fragment_offset
		};
		Ok(header)
	}
}

/// Pushes an IPv4Header onto packets moving down the network stack.
/// Pops off an IPv4Header header for packets moving up the stack.
pub struct IPv4Component
{
	data: ThreadData,

	/// Listens for "send_down" events.
	pub upper_in: InPort<(InternetInfo, SocketOptions, Packet)>,	
	pub upper_out: OutPort<(InternetInfo, Packet)>,

	/// Listens for "send_up" events.
	pub lower_in: InPort<(LinkInfo, Packet)>,
	pub lower_out: OutPort<(IPv4Header, Packet)>,
}

impl IPv4Component
{
	pub fn new(sim: &mut Simulation, parent_id: ComponentID) -> IPv4Component
	{
		let (id, data) = sim.add_active_component("IPv4", parent_id);
		IPv4Component {
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
					let (iinfo, options, mut packet) = event.take_payload::<(InternetInfo, SocketOptions, Packet)>();
					let header = IPv4Header::with_internet(&iinfo, &options);
					header.push(&mut packet);
					self.lower_out.send_payload(&mut effector, &event.name, (header, packet));
				},
				"send_up" => {
					let (_, mut packet) = event.take_payload::<(LinkInfo, Packet)>();
					match IPv4Header::pop(&mut packet) {
						Ok(header) => {
							let iinfo = InternetInfo::new(header.protocol, IPAddress::IPv4(header.src_addr), IPAddress::IPv4(header.dst_addr));
							self.upper_out.send_payload(&mut effector, &event.name, (iinfo, packet));
						},
						Err(mesg) => log_warning!(effector, "pop failed: {}", mesg)
					}
				}
			);
		});
	}
}

mod tests
{
    #[cfg(test)]
	use super::*;

    #[test]
    fn ipv4_header_pushing()
	{
		let mut packet = Packet::new("test packet", "1");
		let payload = "hello world".to_string();
		packet.push_bytes(payload.as_bytes());

		let src_ip = [127, 0, 0, 1];
		let dst_ip = [10, 0, 0, 255];
		let options = SocketOptions::with_addr(IPAddress::IPv4(dst_ip));
		let header1 = IPv4Header::new(EXPERIMENTAL1, src_ip, dst_ip, &options);
		header1.push(&mut packet);
		//println!("{:?}", packet);

		match IPv4Header::pop(&mut packet) {
			Ok(header2) => {
				assert_eq!(header1.dscp, header2.dscp);
				assert_eq!(header1.ecn, header2.ecn);
				assert_eq!(header1.identification, header2.identification);
				assert_eq!(header1.dont_fragment, header2.dont_fragment);
				assert_eq!(header1.more_fragments, header2.more_fragments);
				assert_eq!(header1.fragment_offset, header2.fragment_offset);
				assert_eq!(header1.ttl, header2.ttl);
				assert_eq!(header1.protocol, header2.protocol);

				assert_eq!(header1.src_addr[0], header2.src_addr[0]);
				assert_eq!(header1.src_addr[1], header2.src_addr[1]);
				assert_eq!(header1.src_addr[2], header2.src_addr[2]);
				assert_eq!(header1.src_addr[3], header2.src_addr[3]);

				assert_eq!(header1.dst_addr[0], header2.dst_addr[0]);
				assert_eq!(header1.dst_addr[1], header2.dst_addr[1]);
				assert_eq!(header1.dst_addr[2], header2.dst_addr[2]);
				assert_eq!(header1.dst_addr[3], header2.dst_addr[3]);
			}
			Err(mesg) => assert!(false, "IPv4Header::pop failed: ".to_string() + &mesg)
		}

		let len = packet.len();
		let data = packet.pop_bytes(len);
		match str::from_utf8(data.as_slice()) {
			Ok(text) => assert_eq!(payload, text),
			Err(mesg) => assert!(false, format!("IPv4Header::pop_payload failed: {}", mesg))
		}
    }
}
