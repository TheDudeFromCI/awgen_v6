//! This module adds the `awgen` extension to the Deno runtime, which provides
//! core functionality to the Javascript backend for Awgen.

use std::future::Future;
use std::time::Duration;

use bevy::prelude::*;
use rustyscript::deno_core::{extension, op2};
use smol::Timer;

extension!(
    awgen,
    ops = [op_log, op_sleep_async],
    esm_entry_point = "awgen:core",
    esm = [ dir "js", "awgen:core" = "awgen_core.js"],
);

/// A simple logging operation that logs a message to the console.
#[op2(fast)]
fn op_log(#[string] message: String) {
    info!("SCRIPT: {}", message);
}

/// A simple operation that sleeps for a given number of milliseconds.
#[op2(async)]
fn op_sleep_async(#[bigint] ms: i64) -> impl Future<Output = ()> {
    let dur = Duration::from_millis(ms as u64);

    async move {
        Timer::after(dur).await;
    }
}
