//! Utilities for sccache/tests
//!
//! Any copyright is dedicated to the Public Domain.
//! http://creativecommons.org/publicdomain/zero/1.0/

#[macro_use]
extern crate log;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn find_sccache_binary() -> PathBuf {
    // Older versions of cargo put the test binary next to the sccache binary.
    // Newer versions put it in the deps/ subdirectory.
    let exe = env::current_exe().unwrap();
    let this_dir = exe.parent().unwrap();
    let dirs = &[&this_dir, &this_dir.parent().unwrap()];
    dirs
        .iter()
        .map(|d| d.join("sccache").with_extension(env::consts::EXE_EXTENSION))
        .filter_map(|d| fs::metadata(&d).ok().map(|_| d))
        .next()
        .expect(&format!("Error: sccache binary not found, looked in `{:?}`. Do you need to run `cargo build`?", dirs))
}

pub fn stop(sccache: &Path) {
    //TODO: should be able to use Assert::ignore_status when that is released.
    let output = Command::new(&sccache)
        .arg("--stop-server")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .unwrap();
    trace!("stop-server returned {}", output.status);
}