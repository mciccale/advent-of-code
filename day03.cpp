#include "utils.hpp"
#include <iostream>
#include <ranges>

int main()
{
  auto lines = read_lines(std::cin);
  for (std::string_view s : lines | std::views::all)
    std::cout << s << "\n";
  return 0;
}
