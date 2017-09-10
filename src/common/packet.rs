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
use std::collections::VecDeque;
use std::fmt;

/// A sequence of bytes sent down the network stack, over a wire or the air,
/// and back up the network stack.
pub struct Packet
{
	name: String,
	id: String,
	offset: usize,	// used when popping to avoid shifting bytes down
	payload: VecDeque<u8>,
}

/// Used to assemble a header to be pushed onto a [`Packet`].
pub struct Header
{
	pub data: Vec<u8>,
}

impl Packet
{
	pub fn new(name: &str, id: &str) -> Self
	{
		assert!(!name.is_empty());
		assert!(!id.is_empty());

		let payload = VecDeque::with_capacity(32);
		Packet{name: name.to_string(), id: id.to_string(), payload, offset: 0}
	}

	/// Arbitrary name of the packet, e.g. "ICMP Ping". If the packet is fragmented
	/// the fragments will have names like "ICMP Ping{1 of 4}". If packets are aggregated
	/// the new packet will have a name like "ICMP Ping/RTP/RTP".
	pub fn name(&self) -> &str
	{
		&self.name
	}

	/// Unique identifier for the packet, e.g. "#>12.56". The punctuation serves to make it
	/// easier to search for a particular id. The first number is the ID of the component that
	/// originated the packet. The second number is a monotonically increasing value associated
	/// with the component.
	///
	/// If the packet is fragmented the ids will have names like "#>12.56{1 of 4}". If packets
	/// are aggregated the new packet will have an id like "#>12.56/#>12.57/#>42.10".
	pub fn id(&self) -> &str
	{
		&self.id
	}

	/// Packet data in network endian byte order.
	pub fn payload(&self) -> &VecDeque<u8>
	{
		&self.payload
	}

	/// Returns true if all of the payload has been popped off.
	pub fn is_empty(&mut self) -> bool
	{
		self.offset == self.payload.len()
	}

	pub fn len(&self) -> usize
	{
		self.payload.len() - self.offset
	}

	/// This is what components within the network stack use.
	pub fn push_header(&mut self, header: &Header)
	{
		assert!(self.offset == 0, "mixing pushs and pops isn't supported");

		self.payload.reserve(header.data.len());
		for b in header.data.iter().rev() {
			self.payload.push_front(*b);
		}
	}

	/// Apps can use this to push payloads onto a packet.
	pub fn push_bytes(&mut self, data: &[u8])
	{
		assert!(self.offset == 0, "mixing pushs and pops isn't supported");

		self.payload.extend(data);
	}

	/// Removes data from the front of the payload.
	pub fn pop8(&mut self) -> u8
	{
		self.offset += 1;
		self.payload[self.offset - 1]
	}

	pub fn pop16(&mut self) -> u16
	{
		let b0 = self.pop8() as u16;
		let b1 = self.pop8() as u16;
		(b0 << 8) | b1
	}

	pub fn pop32(&mut self) -> u32
	{
		let b0 = self.pop8() as u32;
		let b1 = self.pop8() as u32;
		let b2 = self.pop8() as u32;
		let b3 = self.pop8() as u32;
		(b0 << 24) | (b1 << 16) | (b2 << 8) | b3
	}

	// TODO: more efficient to return a slice tho I'm not sure how well that'd play with the borrow checker
	pub fn pop_bytes(&mut self, len: usize) -> Vec<u8>
	{
		let mut result = Vec::with_capacity(len);

		for _ in 0..len {
			result.push(self.pop8());
		}

		result
	}

	pub fn checksum(&self, len: usize) -> u16
	{
		super::checksum::checksum(&self.payload, len)
	}
}

// TODO: Ideally we would add headers directly to Packet but I wasn't able to figure out an
// efficient way to do that with safe code (we'd need something like a method to add N default
// constructed elements to the start of a Vec or VecDeque).
impl Header
{
	pub fn new() -> Self
	{
		let data = Vec::with_capacity(20);
		Header{data}
	}

	pub fn with_capacity(capacity: usize) -> Self
	{
		let data = Vec::with_capacity(capacity);
		Header{data}
	}

	// Adds data to the end of the header.
	pub fn push8(&mut self, data: u8)
	{
		self.data.push(data);
	}

	/// Converts data to network endian and adds it to the header.
	pub fn push16(&mut self, data: u16)
	{
		self.data.push((data >> 8) as u8);
		self.data.push((data & 0xFF) as u8);
	}

	pub fn push32(&mut self, data: u32)
	{
		self.data.push((data >> 24) as u8);
		self.data.push(((data >> 16) & 0xFF) as u8);
		self.data.push(((data >> 8) & 0xFF) as u8);
		self.data.push((data & 0xFF) as u8);
	}

	pub fn push_bytes(&mut self, data: &[u8])
	{
		self.data.extend(data);
	}

	pub fn checksum(&self) -> u16
	{
		super::checksum::checksum(&self.data, self.data.len())
	}
}

impl fmt::Debug for Packet 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		// TODO: Use a helper (and improve formatting, including ASCII version of the bytes).
		let mut bytes = String::with_capacity(3*self.len());
		for i in self.offset..self.payload.len() {
			bytes.push_str(&format!(" {:02X}", self.payload[i]));
		}

        write!(f, "{} {}{}", self.name, self.id, bytes)
    }
}

impl fmt::Debug for Header 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		let mut bytes = String::with_capacity(3*self.data.len());
		for i in 0..self.data.len() {
			bytes.push_str(&format!("{:02X} ", self.data[i]));
		}

        write!(f, "{}", bytes)
    }
}

mod tests
{
    #[cfg(test)]
	use super::*;

    #[test]
    fn packet_pushing()
	{
		let mut packet = Packet::new("test packet", "1");

		let mut header1 = Header::new();
		header1.push8(4);
		header1.push8(5);
		header1.push16(0x0708);
		packet.push_header(&header1);

		let mut header2 = Header::new();
		header2.push8(104);
		packet.push_header(&header2);

		// header2 will have been pushed to the front
		let b = packet.pop8();
		assert_eq!(104, b);

		let b = packet.pop8();
		assert_eq!(4, b);

		let b = packet.pop8();
		assert_eq!(5, b);

		let hw = packet.pop16();
		assert_eq!(0x0708, hw);

		assert!(packet.is_empty())
    }
}
