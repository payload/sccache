@echo off

set vcvarsall_14="C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\vcvarsall.bat"
set vcvarsall_15C="C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Auxiliary\Build\vcvarsall.bat"
set vcvarsall_15P="C:\Program Files (x86)\Microsoft Visual Studio\2017\Professional\VC\Auxiliary\Build\vcvarsall.bat"
set vcvarsall_15E="C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Auxiliary\Build\vcvarsall.bat"

echo recreate test-cmake\build
rmdir /s/q test-cmake\build
mkdir test-cmake\build
cd test-cmake\build

if        exist %vcvarsall_15E% ( call %vcvarsall_15E% x64
) else if exist %vcvarsall_15P% ( call %vcvarsall_15P% x64
) else if exist %vcvarsall_15C% ( call %vcvarsall_15C% x64
) else if exist %vcvarsall_14% ( call %vcvarsall_14% x64
) else (
    echo Could not find a Visual Studio vcvarsall.
    echo Please edit this script or run it after running vcvarsall manually.
    exit /b -1
)

set PATH=%CL_EXE_DIR%;%PATH%

cmake .. -G Ninja -DCMAKE_CXX_COMPILER=cl.exe -DCMAKE_C_COMPILER=cl.exe

cd ..
cd ..