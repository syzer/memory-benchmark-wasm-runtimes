#![no_std]

#[cfg(feature = "engine-wasmtime")]
pub mod wasmtime;

#[cfg(feature = "engine-wasmtime")]
mod wasmtime_platform;
