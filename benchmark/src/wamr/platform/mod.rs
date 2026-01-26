//! Module for implementing the platform-specific (in this case bare-metal embassy) functions used by Wamr

mod allocation;
mod bsearch;
mod cache;
mod math;
mod memory_mapping;
// mod printing;
mod basic;
mod quicksort;
mod stack_management;
mod strings;

pub use stack_management::register_stack_boundary;
