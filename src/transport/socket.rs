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

/// QoS is a big mess and the semantics have changed from ToS to QoS to DSCP. We follow
/// mac82011's lead (see https://wireless.wiki.kernel.org/en/developers/documentation/mac80211/queues)
/// and map QoS to one of four queues which each have different priority levels.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum QoS	// See 8.4.2.31 EDCA Parameter Set element 
{
	/// Lowest priority
	Background = 32,

	/// Default priority
	BestEffort = 0,

	// High priority
	Video = 128,

	// Highest priority
	Voice = 192,
}

// See http://man7.org/linux/man-pages/man7/ip.7.html and https://linux.die.net/man/7/socket
pub struct SocketOptions
{
	/// This controls packet precedence when the MAC queues start backing up. Equivalent to 
	/// Linux's SO_PRIORITY option.
	pub qos: QoS,
		
	/// Time to live: maximum number of hops the packet is allowed to travel. Defaults to 255
	/// for unicast and 1 for multicast (which is what Linux 2.4 does). Equivalent to Linux's
	/// IP_TTL and IP_MULTICAST_TTL options.
	pub ttl: u8,

	/// If this is set and the packet is too large for a link then it will be dropped and a
	/// ICMP Fragmentation Needed packet will be sent back. Equivalent to the IP_DONTFRAG (BSD) 
	/// and IP_MTU_DISCOVER (linux) options.
	pub dont_fragment: bool,
}

// See http://elixir.free-electrons.com/linux/latest/source/include/net/sock.h#L118
// pub struct Socket
// {
// 	/// TCP, UDP, IGMP, OSPF, etc.
// 	pub protocol: u8,
	
// 	/// Address of an interface on the local machine.
// 	pub source_addr: IPAddress,
	
// 	/// The destination of the packet.
// 	pub dest_addr: IPAddress,

// 	pub options: SocketOptions,
// }

impl SocketOptions
{
	pub fn with_addr(_: IPAddress) -> Self
	{	
		SocketOptions{qos: QoS::BestEffort, ttl: 255, dont_fragment: false}	// TODO: set ttl to 1 for multicast
	}
}

// impl Socket
// {
// 	pub fn new(protocol: u8, dest_addr: IPAddress) -> Socket
// 	{	
// 		assert!(protocol != RESERVED);

// 		let options = SocketOptions::with_addr(dest_addr);
// 		Socket {
// 			protocol, 
// 			source_addr: IPAddress::IPv4([10, 0, 0, 1]),	// TODO: set this for real
// 			dest_addr,
// 			options,
// 		}
// 	}
// }
