//! A simple VGA Mode 13h graphics driver implementation for embedded systems
//!
//! This module provides functionality to initialize and interact with VGA Mode 13h
//! (320x200 with 256 colors) graphics mode.
//!
//! [en] This driver allows drawing text and primitive shapes on a VGA display
//! [pt-br] Este driver permite desenhar texto e formas primitivas em um display VGA

#![no_std] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
mod init;
mod libs;
mod drivers;
use init::start::init;

/// Main entry point for the application
///
/// Initializes the VGA graphics mode and draws text and shapes
///
/// [en] This function never returns and handles all VGA display operations
/// [pt-br] Esta função nunca retorna e lida com todas as operações do display VGA
fn start() -> ! {
  init();
}
