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
use link::link_helpers::*;
use score::*;
// use std::str;
use std::thread;
// use std::u16;
// use transport::socket::*;

// Unless otherwise indicated all references are to the 2016 version of "Part 11: Wireless LAN Medium Access Control (MAC) and Physical Layer (PHY) Specifications"
// (i.e. the 802.11 spec).

/// In memory representation of a MAC 802.11 data header.
pub struct Mac80211DataFrame	// see 9.3.2
{
	/// Address the frame originated at.
	pub sa: MacAddress,

	/// Address the frame is being sent from. Often the same as sa.
	pub ta: MacAddress,

	/// Address the frame is being forwarded to. Often the same as da.
	pub ra: MacAddress,

	/// Address the frame is being routed to.
	pub da: MacAddress,

	/// AP address.
	pub bssid: MacAddress,

	/// Sequence number set by the TA.
	pub seq_num: u16,

	// TODO: Lots of stuff missing, e.g. qos, fragmentation, retries, power management, and rate control.
}

impl Mac80211DataFrame
{
	pub fn new(ipv4: &IPv4Header, seq_num: u16) -> Self
	{
		let sa = [0, 0, ipv4.src_addr[0], ipv4.src_addr[1], ipv4.src_addr[2], ipv4.src_addr[3]];
		let ta = [0, 0, ipv4.dst_addr[0], ipv4.dst_addr[1], ipv4.dst_addr[2], ipv4.dst_addr[3]];
		let bssid = [0, 0, 0, 0, 0, 0];

		Mac80211DataFrame {
			sa,
			ta,
			ra: sa,
			da: ta,
			bssid,
			seq_num,
		}
	}

	/// Adds an 802.11 ethernet header to the packet.
	pub fn push(&self, packet: &mut Packet)
	{
		let mut header = Header::with_capacity(30);

		let hw = 0b1000_10_00_00000000;	// frame control, see 9.2.4.1, note that B0 is the low bit in the first byte
		header.push16(hw);

		let hw = 0;						// duration/ID, see 9.2.4.2
		header.push16(hw);

		for &b in self.da.iter() {		// address 1, see 9.3.2.1
			header.push8(b);
		}

		for &b in self.sa.iter() {		// address 2
			header.push8(b);
		}

		for &b in self.bssid.iter() {	// address 3
			header.push8(b);
		}

		let hw = self.seq_num << 4;		// sequence control, see 9.2.4.4.1
		header.push16(hw);

		let hw = 0b0_11_0_0000_00000000;// QoS control, see 9.2.4.5.1
		header.push16(hw);

		packet.push_header(&header);

		// Note that 802.3 has a minimum frame body size but 802.11 does not.

		let crc = crc32(packet);		// FCS (which is always little endian)
		let fcs = [(crc & 0xFF) as u8, (crc >> 8 & 0xFF) as u8, (crc >> 16 & 0xFF) as u8, (crc >> 24 & 0xFF) as u8];
		packet.push_back_bytes(&fcs);
	}

	/// Removes an 802.11 ethernet header from the packet.
	pub fn pop(packet: &mut Packet) -> Result<Mac80211DataFrame, String>
	{
		// When sending the crc includes everything but the crc itself (the FCS field).
		// When receiving the crc includes the FCS and, because of the magic of modulo
		// arithmetic, a valid frame's crc will always be 0xC704DD7B.
		let crc = reverse32(!crc32(packet));
		if crc != 0xC704DD7B {
			return Err("Checksum error".to_string())
		}

		let frame_control = packet.pop16();
		if frame_control & 0b11 != 0 {			// 9.2.4.1.2
			return Err("Version isn't zero".to_string())
		}
		let _duration = packet.pop16();

		let addr1 = [packet.pop8(), packet.pop8(), packet.pop8(), packet.pop8(), packet.pop8(), packet.pop8()];
		let addr2 = [packet.pop8(), packet.pop8(), packet.pop8(), packet.pop8(), packet.pop8(), packet.pop8()];
		let addr3 = [packet.pop8(), packet.pop8(), packet.pop8(), packet.pop8(), packet.pop8(), packet.pop8()];

		let sn = packet.pop16();
		let _qos = packet.pop16();

		let _ = packet.pop_back8();	// fcs (we used this when we computed the crc)
		let _ = packet.pop_back8();
		let _ = packet.pop_back8();
		let _ = packet.pop_back8();

		Ok(Mac80211DataFrame {
			sa: addr2,
			ta: addr2,
			ra: addr1,
			da: addr1,
			bssid: addr3,
			seq_num: sn,
		})
	}
}

// TODO: Need an enum to encapsulate the various frame types

/// Medium Access Control for 802.11 wireless radios.
pub struct Mac80211Component
{
	data: ThreadData,

	/// Listens for "send_down" events.
	pub upper_in: InPort<(IPv4Header, Packet)>,	
	pub upper_out: OutPort<(LinkInfo, Packet)>,

	/// Listens for "send_up" events.
	pub lower_in: InPort<Packet>,
	pub lower_out: OutPort<Packet>,
}

impl Mac80211Component
{
	pub fn new(sim: &mut Simulation, parent_id: ComponentID) -> Self
	{
		let (id, data) = sim.add_active_component("Mac80211", parent_id);
		Mac80211Component {
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
					effector.set_int("sn", 0);
				},
				"send_down" => {
					let sn = state.get_int(self.data.id, "sn");
					effector.set_int("num_recv", (sn+1) % 4096);	// sequence number is 12 bits so modulo 4096

					let (ipv4, mut packet) = event.take_payload::<(IPv4Header, Packet)>();
					let header = Mac80211DataFrame::new(&ipv4, sn as u16);
					header.push(&mut packet);
					self.lower_out.send_payload(&mut effector, &event.name, packet);
				},
				"send_up" => {
					let mut packet = event.take_payload::<Packet>();
					match Mac80211DataFrame::pop(&mut packet) {
						Ok(header) => {
							let linfo = LinkInfo::new(0, &header.sa, &header.da);	// TODO: not sure what to do with ether_type since it's not present in 802.11
							self.upper_out.send_payload(&mut effector, &event.name, (linfo, packet));
						},
						Err(mesg) => log_warning!(effector, "pop failed: {}", mesg)
					}
				}
			);
		});
		println!("exiting mac");
	}
}
