//! Data-driven reference test framework for warding
//! against breaking changes.

extern crate gfx_hal as hal;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
extern crate failure;

pub mod gpu;
pub mod raw;
