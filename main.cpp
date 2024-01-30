#include "lib.rs.h"

#include <iostream>

int main() {
    auto strings = rust_from_cpp();
    for (auto const &string: strings) {
        std::cout
                << "string from rust "
                << std::string(string.begin(), string.end())
                << std::endl;
    }
}
