CXX = g++
CXXFLAGS = -std=c++20 -O2 -Wall -Wextra -Wpedantic

%: %.cpp
	$(CXX) $(CXXFLAGS) $< -o $@
