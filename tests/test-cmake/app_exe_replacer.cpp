#include <filesystem>
#include <sstream>
#include <string>
#include <vector>

#include "windows.h"

namespace fs = std::experimental::filesystem;

void
my_exec(const std::string& cmd, std::vector<std::string> args)
{
  std::stringstream ss;
  ss << "\"" << cmd << "\"";
  for (const auto& arg : args) {
    ss << " \"" << arg << "\"";
  }
  auto cmdline = ss.str();

  STARTUPINFO startup_info = { 0 };
  startup_info.cb = sizeof(startup_info);
  PROCESS_INFORMATION process_info = { 0 };

  auto ok = CreateProcess(nullptr,
                          _strdup(cmdline.c_str()),
                          nullptr,
                          nullptr,
                          true,
                          0,
                          nullptr,
                          nullptr,
                          &startup_info,
                          &process_info);
  if (!ok) {
    throw std::exception("Error executing command.");
  } else {
    WaitForSingleObject(process_info.hProcess, INFINITE);
    CloseHandle(process_info.hProcess);
    CloseHandle(process_info.hThread);
  }
}

fs::path
my_get_own_executable_path()
{
  char* path;
  auto err = _get_pgmptr(&path);
  if (err) {
    throw std::exception("Can't get own executable path.");
  }
  return path;
}

int
main(int _argc, char** _argv)
{
  const auto argv = std::vector<std::string>(_argv + 1, _argv + _argc);
  const auto my_path = my_get_own_executable_path();
  const auto replacement_path =
    my_path.parent_path()
      .append(my_path.stem().string() + ".replacement.bat")
      .string();
  my_exec(replacement_path, argv);
  return 0; // ahah
}