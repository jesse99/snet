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
use score::*;
// use std::str;
use std::thread;
use transport::*;

// This is a fn instead of a Fn so that we can avoid generics and, in the future, support
// multiple apps on the same device or chain callbacks. This does mean that users cannot
// pass in closures but users can use the store if they have additional data to pass into
// their callback.
pub type AppCallback = fn (app: &AppComponent, event: &Event, state: &SimState, effector: &mut Effector);

/// Component that makes it easy to instal custom code at the top of the network stack.
pub struct AppComponent
{
	pub data: ThreadData,
	pub callback: Option<AppCallback>,

	pub upper_in: InPort<(InternetInfo, Packet)>,	
	pub upper_out: OutPort<(InternetInfo, SocketOptions, Packet)>,
}

impl AppComponent
{
	pub fn new(sim: &mut Simulation, parent_id: ComponentID) -> AppComponent
	{
		let (id, data) = sim.add_active_component("app", parent_id);
		AppComponent {
			data,
			callback: None,

			upper_in: InPort::with_port_name(id, "upper_in"),
			upper_out: OutPort::new(),
		}
	}
	
	pub fn start(self)
	{		
		thread::spawn(move || {
			for (mut event, state) in self.data.rx.iter() {
				let mut effector = Effector::new();
				match self.callback {
					Some(f) => f(&self, &event, &state, &mut effector),
					None => log_warning!(effector, "dropping {} event", event.name),
				}				
				drop(state);
				let _ = self.data.tx.send(effector);
			}
		});
	}
}
