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
use std::ops::Index;

/// Returns the Internet checksum.
pub fn checksum<I>(bytes: &I, offset: usize, len: usize) -> u16
	where I: Index<usize, Output=u8>
{
	finish_checksum(bytes, offset, len, 0)
}

/// Used to compute the Internet checksum from multiple buffers.
pub fn start_checksum<I>(bytes: &I, offset: usize, len: usize, initial: u32) -> u32
	where I: Index<usize, Output=u8>
{	
	let mut sum = initial;

	let mut i = offset;
	while i+1 < offset+len {
		let word = (*bytes.index(i) as u16) << 8 | (*bytes.index(i+1) as u16);
		sum = sum.wrapping_add(word as u32);
		i += 2;
	}

	assert!(i == offset+len, "must be an even number of bytes");

	sum
}

/// Used to compute the Internet checksum from multiple buffers.
pub fn finish_checksum<I>(bytes: &I, offset: usize, len: usize, initial: u32) -> u16
	where I: Index<usize, Output=u8>
{
	// Based on section 4.1 in https://tools.ietf.org/html/rfc1071
	let mut sum = initial;

	// Add each 16-bit word
	let mut i = offset;
	while i+1 < offset+len {
		let word = (*bytes.index(i) as u16) << 8 | (*bytes.index(i+1) as u16);	// we store the checksum in network endian (aka big endian) so we need to do the loads using network endian
		sum = sum.wrapping_add(word as u32);
		i += 2;
	}

	// Add the left over byte if it exists. Note that for UDP and TCP a pad byte is supposed
	// to be added when the length is odd. Rather than molesting the payload we just shift
	// the left over byte leftwards.
	if i < offset+len {
		let word = (*bytes.index(i) as u16) << 8;
		sum = sum.wrapping_add(word as u32);
	}

	// Fold the 32-bit sum into 16-bits.
	while (sum >> 16) != 0 {
		sum = (sum & 0xFFFF).wrapping_add(sum >> 16);
	}

	// And return the complement.
	!sum as u16
}
