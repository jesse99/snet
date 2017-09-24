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
	// Based on section 4.1 in https://tools.ietf.org/html/rfc1071
	let mut sum: i32 = 0;

	// Add each 16-bit word
	let mut i = offset;
	while i+1 < offset+len {
		let word = (*bytes.index(i) as u16) << 8 | (*bytes.index(i+1) as u16);
		sum = sum.wrapping_add(word as i32);
		i += 2;
	}

	// Add the left over byte if it exists.
	if i < offset+len {
		let word = *bytes.index(i) as u16;
		sum = sum.wrapping_add(word as i32);
	}

	// Fold the 32-bit sum into 16-bits.
	while (sum >> 16) != 0 {
		sum = (sum & 0xFFFF) | (sum >> 16);
	}

	// And return the complement.
	!sum as u16
}
