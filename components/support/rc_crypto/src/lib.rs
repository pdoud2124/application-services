/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

pub mod constant_time;
pub mod digest;
mod error;
mod util;

pub use crate::error::{Error, ErrorKind, Result};
