#include <signal.h>

__sighandler_t signal_wrapper(int interrupt, __sighandler_t function) {
    return signal(interrupt, function);
}