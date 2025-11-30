#pragma once
#include <string>
#include <vector>
#include <fstream>

inline std::vector<std::string> read_lines(std::istream& in)
{
  std::vector<std::string> v;
  std::string s;
  while (std::getline(in, s))
    v.push_back(s);
  return v;
}
