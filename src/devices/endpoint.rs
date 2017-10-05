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
//use internet::protocol_numbers::*;
use link::*;
use physical::*;
use score::*;
// use std::str;
//use std::thread;
use transport::*;
use user::*;

const START_X: f64 = 25.0;
const START_Y: f64 = 5.0;
const DY: f64 = 10.0;

/// Network device that can be used as a source or sink of traffic.
pub struct Endpoint
{
	pub name: String,
	pub id: ComponentID,

	pub app: AppComponent,
	pub udp: UdpComponent,
	pub ipv4: IPv4Component,	// TODO: should be InternetComponent
	pub llc: LlcComponent,
	pub mac: Mac80211Component,

	pub pcap: PcapComponent,
}

impl Endpoint
{
	pub fn new(name: &str, sim: &mut Simulation, parent_id: ComponentID) -> Self
	{
		let id = sim.add_component(name, parent_id);

		let app = AppComponent::new(sim, id);
		let udp = UdpComponent::new(sim, id);
		let ipv4 = IPv4Component::new(sim, id);
		let llc = LlcComponent::new(sim, id);
		let mac = Mac80211Component::new(sim, id);
		let pcap = PcapComponent::new(sim, id);
		Endpoint {
			name: name.to_string(),
			id,
			app,
			udp,
			ipv4,
			llc,
			mac,

			pcap,
		}
	}

	pub fn start(mut self, sim: &mut Simulation, medium: &mut Medium80211Component)	// TODO: use a trait for the medium
	{
		// Wire together the components.
		self.app.lower_out.connect_to(&self.udp.upper_in);
		self.udp.upper_out.connect_to(&self.app.lower_in);

		self.udp.lower_out.connect_to(&self.ipv4.upper_in);
		self.ipv4.upper_out.connect_to(&self.udp.lower_in);

		self.ipv4.lower_out.connect_to(&self.llc.upper_in);
		self.llc.upper_out.connect_to(&self.ipv4.lower_in);

		self.llc.lower_out.connect_to(&self.mac.upper_in);
		self.mac.upper_out.connect_to(&self.llc.lower_in);

		self.mac.pcap_out.connect_to(&self.pcap.ieee80211_in);
		medium.connect(&mut self.mac.lower_out, &self.mac.lower_in);
		
		// Spin up the threads.
		self.app.start();
		self.udp.start();
		self.ipv4.start();
		self.llc.start();
		self.mac.start();
		self.pcap.start();
		
		// Set our state.
		let mut effector = Effector::new();
		{
		let (_, root) = sim.components.get_root();
			effector.set_string("display-name", &self.name);
			effector.set_float("display-location-x", START_X);
			effector.set_float("display-location-y", START_Y + DY*(root.children.len()) as f64);
		}
		sim.apply(self.id, effector);
	}
}
