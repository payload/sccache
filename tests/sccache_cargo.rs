//! System tests for compiling Rust code with cargo.
//!
//! Any copyright is dedicated to the Public Domain.
//! http://creativecommons.org/publicdomain/zero/1.0/

extern crate assert_cli;
extern crate chrono;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate tempdir;
extern crate testutils;

use std::env;
use std::fs;
use std::path::{Path};

use assert_cli::{Assert, Environment};
use env_logger::LogBuilder;
use chrono::Local;
use tempdir::TempDir;
use testutils::{find_sccache_binary, stop};

/// Test that building a simple Rust crate with cargo using sccache results in a cache hit
/// when built a second time.
#[test]
#[cfg(not(target_os="macos"))] // test currently fails on macos
fn test_rust_cargo() {
    drop(LogBuilder::new()
         .format(|record| {
             format!("{} [{}] - {}",
                     Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                     record.level(),
                     record.args())
         })
        .parse(&env::var("RUST_LOG").unwrap_or_default())
        .init());
    let cargo = env!("CARGO");
    debug!("cargo: {}", cargo);
    let sccache = find_sccache_binary();
    debug!("sccache: {:?}", sccache);
    let crate_dir = Path::new(file!()).parent().unwrap().join("test-crate");
    // Ensure there's no existing sccache server running.
    trace!("sccache --stop-server");
    stop(&sccache);
    // Create a temp directory to use for the disk cache.
    let tempdir = TempDir::new("sccache_test_rust_cargo").unwrap();
    let cache_dir = tempdir.path().join("cache");
    fs::create_dir(&cache_dir).unwrap();
    let cargo_dir = tempdir.path().join("cargo");
    fs::create_dir(&cargo_dir).unwrap();
    let env = Environment::inherit().insert("SCCACHE_DIR", &cache_dir);
    // Start a new sccache server.
    trace!("sccache --start-server");
    Assert::command(&[&sccache.to_string_lossy()])
        .with_args(&["--start-server"]).with_env(env).succeeds().unwrap();
    // `cargo clean` first, just to be sure there's no leftover build objects.
    let env = Environment::inherit()
        .insert("RUSTC_WRAPPER", &sccache)
        .insert("CARGO_TARGET_DIR", &cargo_dir);
    let a = Assert::command(&[&cargo])
        .with_args(&["clean"]).with_env(&env).current_dir(&crate_dir).succeeds();
    trace!("cargo clean: {:?}", a);
    a.unwrap();
    // Now build the crate with cargo.
    let a = Assert::command(&[&cargo])
        .with_args(&["build", "--color=never"]).with_env(&env).current_dir(&crate_dir)
        .stderr().doesnt_contain("\x1b[").succeeds();
    trace!("cargo build: {:?}", a);
    a.unwrap();
    // Clean it so we can build it again.
    let a = Assert::command(&[&cargo])
        .with_args(&["clean"]).with_env(&env).current_dir(&crate_dir).succeeds();
    trace!("cargo clean: {:?}", a);
    a.unwrap();
    let a = Assert::command(&[&cargo])
        .with_args(&["build", "--color=always"]).with_env(&env).current_dir(&crate_dir)
        .stderr().contains("\x1b[").succeeds();
    trace!("cargo build: {:?}", a);
    a.unwrap();
    // Now get the stats and ensure that we had a cache hit for the second build.
    trace!("sccache --show-stats");
    Assert::command(&[&sccache.to_string_lossy()])
        .with_args(&["--show-stats", "--stats-format=json"])
        .stdout().contains(r#""cache_hits":1"#).succeeds().execute()
        .expect("Should have had 1 cache hit");
    trace!("sccache --stop-server");
    stop(&sccache);
}
