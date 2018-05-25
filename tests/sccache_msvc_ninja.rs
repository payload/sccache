use std::process::Command;

// - cmd: cd tests\test-cmake && cmake . -G Ninja -DCMAKE_CXX_COMPILER=cl.exe -DCMAKE_C_COMPILER=cl.exe && cmake --build .
#[test]
fn test_cmake_configure() {
    let status = Command::new("cmd")
        .current_dir("tests")
        .args(&["/C", "test_cmake_configure.bat"])
        .status()
        .expect("failed to execute process"); 
    assert!(status.success());
}