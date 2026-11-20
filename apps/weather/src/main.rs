#![cfg_attr(target_os = "none", no_std, no_main)]

#[cfg_attr(not(target_os = "none"), allow(dead_code))]
mod model;

#[cfg(not(target_os = "none"))]
fn main() {}
