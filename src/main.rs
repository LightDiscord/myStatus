/**
 * Author: LightDiscord <arnaud@lightdiscord.me>
 * Date  : 16.06.2018
 */

#[macro_use]
extern crate error_chain;
extern crate tokio;

pub mod error;

use error::*;
use error_chain::ChainedError;
use std::time::{ Duration, Instant, SystemTime };
use tokio::prelude::*;
use tokio::timer::Interval;

fn main() {
    println!("Hello, world!");

    let task = create_interval()
        .map_err(|error| println!("{}", error.display_chain().to_string()));

    tokio::run(task);
}

fn create_interval () -> impl Future<Item = (), Error = error::Error> {
    let now = Instant::now();

    Interval::new(now, Duration::from_secs(1))
            .map_err(|error| error::Error::from(error))
            .for_each(|instant| Bar::build(instant))
}

struct Bar;

impl Bar {
    pub fn build (instant: Instant) -> Result<()> {
        println!("fire; instant={:?}", instant);

        let now = SystemTime::now();
        let now = now.duration_since(SystemTime::UNIX_EPOCH)?;
        let now = now.as_secs();

        println!("seconds: {}", now % 60);

        Ok(())
    }
}
