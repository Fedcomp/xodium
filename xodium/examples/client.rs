//! Example client for X server.
//! Currenly can only connect to unix socket without any setup.
//! In real life scenario i would recomment to use anyhow as
//! a main resulting type

use std::io;

fn main() -> Result<(), io::Error> {
    let _connection = xodium::connect_default()?;
    Ok(())
}
