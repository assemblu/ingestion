#include <iostream>
#include <thread>
#include <atomic>
#include <signal.h>
#include <iomanip>
#include <chrono>

std::atomic<bool> kill_switch_triggered(false);

void signal_handler(int signum) {
    std::cout << "Received signal: " << signum << std::endl;
    kill_switch_triggered.store(true);
}

void setup_signals() {
    struct sigaction sa;
    sa.sa_handler = signal_handler;
    sigemptyset(&sa.sa_mask);
    sa.sa_flags = 0;
    sigaction(SIGINT, &sa, nullptr);
    sigaction(SIGTERM, &sa, nullptr);
}


int main() {
    setup_signals();



    return 0;
}
