extern crate assert_cli;
extern crate testutils;

use std::env;
use std::fs;
use std::path::{Path};
use std::process::{Command};

use assert_cli::{Assert, Environment};
use testutils::{find_sccache_binary};

// - cmd: cd tests\test-cmake && cmake . -G Ninja -DCMAKE_CXX_COMPILER=cl.exe -DCMAKE_C_COMPILER=cl.exe && cmake --build .
#[test]
fn test_cmake_configure() {
    let here_rel = Path::new(file!()).parent().unwrap();
    let here = fs::canonicalize(here_rel).unwrap();
    let cl_exe = here.join("cl.exe");
    let sccache_path = find_sccache_binary();
    let sccache = sccache_path.to_str().unwrap();
    
    Command::new(sccache)
        .arg("--stop-server")
        .output()
        .unwrap();
    Command::new(sccache)
        .arg("--start-server")
        .status()
        .unwrap();
    Command::new(sccache)
        .arg("--show-stats")
        .status()
        .unwrap();

    fs::hard_link(sccache, cl_exe)
        .unwrap_or_default();

    Command::new("cmd")
        .args(&["/C", "test_cmake_configure.bat"])
        .current_dir("tests")
        .env("CL_EXE_DIR", here)
        .status()
        .unwrap();

    Command::new(sccache)
        .arg("--show-stats")
        .status()
        .unwrap();


    Command::new("cmd")
        .args(&["/C", "test_cmake_configure.bat"])
        .current_dir("tests")
        .env("CL_EXE_DIR", here)
        .status()
        .unwrap();

    Command::new(sccache)
        .arg("--show-stats")
        .status()
        .unwrap();
}