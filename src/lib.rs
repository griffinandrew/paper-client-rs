/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */


#![cfg_attr(feature = "allocator_api", feature(allocator_api))]

#[cfg(feature = "allocator_api")]
pub mod allocator;
pub use crate::allocator::*;

pub mod paper_client;
pub use crate::paper_client::*;

pub mod error;
pub use error::PaperClientError;

pub mod paper_pool;
pub use crate::paper_pool::*;

pub mod policy;
pub use crate::policy::*;

pub mod status;
pub use crate::status::*;

mod value;
pub use crate::value::*;

mod arg;
mod addr;
mod command;

