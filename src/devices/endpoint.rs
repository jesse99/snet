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
use score::*;
// use std::str;
//use std::thread;
//use transport::socket::*;
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
	pub ipv4: IPv4Component,	// TODO: should be InternetComponent
	pub mac: Mac80211Component,
}

impl Endpoint
{
	pub fn new(name: &str, sim: &mut Simulation, parent_id: ComponentID) -> Self
	{
		let id = sim.add_component(name, parent_id);

		let app = AppComponent::new(sim, id);
		let ipv4 = IPv4Component::new(sim, id);
		let mac = Mac80211Component::new(sim, id);
		Endpoint {
			name: name.to_string(),
			id,
			app,
			ipv4,
			mac,
		}
	}

	pub fn start(mut self, sim: &mut Simulation)
	{
		// Wire together the components.
		self.app.upper_out.connect_to(&self.ipv4.upper_in);
		self.ipv4.upper_out.connect_to(&self.app.upper_in);

		self.ipv4.lower_out.connect_to(&self.mac.upper_in);
		self.mac.upper_out.connect_to(&self.ipv4.lower_in);
	
		// Spin up the threads.
		self.app.start();
		self.ipv4.start();
		self.mac.start();
		
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

	pub fn connect(&mut self, other: &mut Endpoint)
	{
		self.mac.lower_out.connect_to(&other.mac.lower_in);
		other.mac.lower_out.connect_to(&self.mac.lower_in);
	}
}
