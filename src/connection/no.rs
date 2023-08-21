// Jetporch
// Copyright (C) 2023 - Michael DeHaan <michael@michaeldehaan.net> + contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// long with this program.  If not, see <http://www.gnu.org/licenses/>.

use crate::connection::connection::{Connection};
use crate::connection::factory::ConnectionFactory;
use crate::playbooks::context::PlaybookContext;
use crate::inventory::hosts::Host;
use crate::tasks::handle::TaskHandle;
use crate::tasks::request::TaskRequest;
use crate::tasks::response::TaskResponse;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

pub struct NoFactory {}

impl NoFactory { 
    pub fn new() -> Self {
        Self {}
    }
}

impl ConnectionFactory for NoFactory {
    fn get_connection(&self, _context: &Arc<RwLock<PlaybookContext>>, _host: &Arc<RwLock<Host>>) -> Result<Arc<Mutex<dyn Connection>>,String> {
        return Ok(Arc::new(Mutex::new(NoConnection::new())));
    }
}

pub struct NoConnection {
}

impl NoConnection {
    pub fn new() -> Self {
        Self { }
    }
}

impl Connection for NoConnection {

   fn connect(&mut self) -> Result<(),String> {
       return Ok(());
   }

   fn run_command(&self, _handle: &TaskHandle, _request: &Arc<TaskRequest>, _cmd: &String) -> Result<Arc<TaskResponse>,Arc<TaskResponse>> {
       panic!("NoConnection may only be used for syntax checking");
   }

   // FIXME: this signature will change
   fn put_file(&self, _data: String, _remote_path: String, _mode: Option<i32>) {
       panic!("NoConnection may only be used for syntax checking");
   }
}