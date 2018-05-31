#import <stdio.h>
#include <vector>
#include <string>

std::vector<std::string> buffer;

void entered_func(std::string func_name) {
  buffer.push_back("entering function " + func_name);
}

void exited_func (std::string func_name) {
  buffer.push_back("exiting function " + func_name);
}
