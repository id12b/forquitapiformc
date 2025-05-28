#include <iostream>

extern "C" {
    void load_mods();
}

int main() {
    load_mods();
    std::cout << "Rust mod loader called from C++!" << std::endl;
    return 0;
}
