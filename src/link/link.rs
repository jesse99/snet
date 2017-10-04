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
use link::ether_type::*;

pub type MacAddress = [u8; 6];

/// This, and and [`Packet`], are the types used to communicate between
/// the internet and link layers.
pub struct LinkInfo
{
	/// See https://en.wikipedia.org/wiki/EtherType
	pub ether_type: EtherType,
	
	/// The sender of the packet.
	pub src_addr: MacAddress,
	
	/// The destination of the packet.
	pub dst_addr: MacAddress,
}

impl LinkInfo
{
	pub fn new(ether_type: EtherType, src_addr: &MacAddress, dst_addr: &MacAddress) -> Self
	{	
		LinkInfo {ether_type, src_addr: *src_addr, dst_addr: *dst_addr}
	}
}
