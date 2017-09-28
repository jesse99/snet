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
// use internet::*;
// use link::link::*;
// use link::link_helpers::*;
use score::*;
use std::fs;
use std::io;
use std::io::Write;
use std::thread;
use time;

/// Component used to write our pcap files for use by tools such as Wireshark or tshark.
/// There are several component settings that affect how this is done:
/// - **count** is the maximum number of packets to write out. Defaults to 100_000.
/// - **path** is the file in which the pcap is stored. If this is empty then no pcap is generated. Defaults to empty.
/// - **promiscuous** if 1 save all frames that were seen. Otherwise only save frames addressed to this device. Defaults to 0. TODO: not implemented
/// - **snap_length** is the maximum number of packet bytes to write out. Defaults to 65536.
/// 
pub struct PcapComponent	
{
	data: ThreadData,

	/// Listens for "send_down" events.
	pub upper_in: InPort<Packet>,	
	pub upper_out: OutPort<Packet>,

	/// Listens for "send_up" events.
	pub lower_in: InPort<Packet>,
	pub lower_out: OutPort<Packet>,
}

// TODO: add direction setting? tcpdump uses in, out, and inout for the values
// TODO: add support for radiotap headers?
impl PcapComponent
{
	pub fn new(sim: &mut Simulation, parent_id: ComponentID) -> Self
	{
		let (id, data) = sim.add_active_component("pcap", parent_id);
		PcapComponent {
			data: data,

			upper_in: InPort::with_port_name(id, "upper_in"),
			lower_out: OutPort::new(),

			lower_in: InPort::with_port_name(id, "lower_in"),
			upper_out: OutPort::new(),
		}
	}
	
	pub fn start(self)
	{
		let mut file = Err(io::Error::new(io::ErrorKind::Other, "no path"));
		let mut result = Ok(());
		let mut snap_length = 65535;

		thread::spawn(move || {
			process_events!(self.data, event, state, effector,
				"init 0" => {
					if !state.contains(self.data.id, "count") {
						effector.set_int("count", 100_000);	// we really only need to do all this if we have a path but we always do it so that our state looks nicer in GUIs
					}
					if !state.contains(self.data.id, "promiscuous") {
						effector.set_int("promiscuous", 0);	// TODO: support this (on the rx path)
					}
					if !state.contains(self.data.id, "snap_length") {
						effector.set_int("snap_length", snap_length as i64);
					} else {
						snap_length = state.get_int(self.data.id, "snap_length") as u32;
					}
					if !state.contains(self.data.id, "path") {
						effector.set_string("path", "");
					} else {
						let path = state.get_string(self.data.id, "path");
						file = fs::File::create(&path);
						match file {
							Ok(ref mut f) => {
								result = write_global_header(f, snap_length);
								if let Err(ref e) = result {
									log_error!(effector, "failed to write the global header: {:?}", *e);
								}
							},
							Err(ref e) => log_error!(effector, "failed to create pcap: {:?}", *e)
						}
					}
					effector.set_int("frame", 0);
				},
				"send_down" => {
					let packet = event.take_payload::<Packet>();
					if let Ok(ref mut f) = file {
						if result.is_ok() {
							result = self.write_frame(f, &mut effector, &state, &packet, snap_length);
							if let Err(ref e) = result {
								log_error!(effector, "failed to write a frame: {:?}", *e);
							}
						}
					}
					self.lower_out.send_payload(&mut effector, "send_up", packet);
				},
				"send_up" => {
					let packet = event.take_payload::<Packet>();
					if let Ok(ref mut f) = file {
						if result.is_ok() {
							result = self.write_frame(f, &mut effector, &state, &packet, snap_length);
							if let Err(ref e) = result {
								log_error!(effector, "failed to write a frame: {:?}", *e);
							}
						}
					}
					self.upper_out.send_payload(&mut effector, &event.name, packet);
				}
			);
		});
	}

	fn write_frame(&self, file: &mut fs::File, effector: &mut Effector, state: &SimState, packet: &Packet, snap_length: u32) -> io::Result<()>
	{
		let frame = state.get_int(self.data.id, "frame") + 1;
		let count = state.get_int(self.data.id, "count");
		if frame <= count {
			try!(write_frame_header(file, state.time, snap_length, packet.len() as u32));
			try!(write_frame_body(file, packet, snap_length));
			effector.set_int("frame", frame);
		}
		Ok(())
	}
}

// It doesn't matter what byte order we write this stuff out as, it only matters that we are consistent.
// So we'll just write them out as little endian because that seems to be what most hardware uses nowadays.
// See https://wiki.wireshark.org/Development/LibpcapFileFormat for more.
fn write_global_header<W>(writer: &mut W, snap_length: u32) -> io::Result<()>
	where W: io::Write
{
	try!(write_u32(writer, 0xa1b2c3d4));	// magic number
	try!(write_u16(writer, 2));				// major version number
	try!(write_u16(writer, 4));				// minor version number
	try!(write_u32(writer, 0));				// timezone correction, 0 because our timestamps are GMT
	try!(write_u32(writer, 0));				// timestamp accuracy, all tools set this to zero
	try!(write_u32(writer, snap_length));	// max bytes to record within a packet
	try!(write_u32(writer, 105));			// link layer header type, this is LINKTYPE_IEEE802_11, see http://www.tcpdump.org/linktypes.html for more
	Ok(())
}

fn write_frame_header<W>(writer: &mut W, timestamp: f64, snap_length: u32, len: u32) -> io::Result<()>
	where W: io::Write
{
	let current = time::now_utc().to_timespec();
	let secs = timestamp.floor() + current.sec as f64;
	let usecs = (timestamp - timestamp.floor())*1_000_000.0;

	try!(write_u32(writer, secs as u32));		// seconds
	try!(write_u32(writer, usecs as u32));		// microseconds
	if len < snap_length {						// recorded bytes
		try!(write_u32(writer, len));
	} else {
		try!(write_u32(writer, snap_length));
	}
	try!(write_u32(writer, len));				// actual bytes
	Ok(())
}

fn write_frame_body<W>(writer: &mut W, packet: &Packet, snap_length: u32) -> io::Result<()>
	where W: io::Write
{
	let len = if packet.len() < snap_length as usize {packet.len()} else {snap_length as usize};
	let mut buffer = io::BufWriter::with_capacity(len, writer);
	for i in 0..len {
		try!(buffer.write_all(&[packet.get(i)]));
	}
	Ok(())
}

fn write_u16<W>(writer: &mut W, value: u16) -> io::Result<()>
	where W: io::Write
{
	let bytes = [(value & 0xFF) as u8, (value >> 8) as u8];
	writer.write_all(&bytes)
}

fn write_u32<W>(writer: &mut W, value: u32) -> io::Result<()>
	where W: io::Write
{
	let bytes = [(value & 0xFF) as u8, ((value >> 8) & 0xFF) as u8, ((value >> 16) & 0xFF) as u8, ((value >> 24) & 0xFF) as u8];
	writer.write_all(&bytes)
}
