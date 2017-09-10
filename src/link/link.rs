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
//use common::*;
use internet::*;
// use score::*;
// use std::str;
// use std::thread;
//use transport::socket::*;

/// This, and and [`Packet`], are the types used to communicate between
/// the internet and link layers.
pub struct LinkInfo
{
	/// See https://en.wikipedia.org/wiki/EtherType
	pub ether_type: u8,	// TODO: use an enum for this
	
	/// The sender of the packet.
	pub src_addr: u64,	// TODO: use a real type
	
	/// The destination of the packet.
	pub dst_addr: u64,
}

impl LinkInfo
{
	pub fn new(protocol: u8, src_addr: u64, dst_addr: u64) -> Self
	{	
		assert!(protocol != RESERVED);
		let ether_type = protocol;		// TODO: this isn't right
		LinkInfo {ether_type, src_addr, dst_addr}
	}
}
