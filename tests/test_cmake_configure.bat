@echo off

set vcvarsall_14="C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\vcvarsall.bat"
set vcvarsall_15C="C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Auxiliary\Build\vcvarsall.bat"
set vcvarsall_15P="C:\Program Files (x86)\Microsoft Visual Studio\2017\Professional\VC\Auxiliary\Build\vcvarsall.bat"
set vcvarsall_15E="C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Auxiliary\Build\vcvarsall.bat"

cd test-cmake
mkdir build
cd build

if        exist %vcvarsall_15E% ( call %vcvarsall_15E% x64
) else if exist %vcvarsall_15P% ( %vcvarsall_15P% x64
) else if exist %vcvarsall_15C% ( %vcvarsall_15C% x64
) else if exist %vcvarsall_14% ( %vcvarsall_14% x64
) else (
    echo Could not find a Visual Studio vcvarsall.
    echo Please edit this script or run it after running vcvarsall manually.
    exit /b -1
)

set PATH=%CL_EXE_DIR%;%PATH%

where cl
echo %PATH%

cmake .. -G Ninja -DCMAKE_CXX_COMPILER=cl.exe -DCMAKE_C_COMPILER=cl.exe

cd ..
cd ..