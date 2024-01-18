#include "minicoro.h"


typedef long int seL4_CPtr;

class Notification {
    public:
        void send_signal();
};

class Runtime {
    public:
        Runtime(seL4_CPtr send_notification, seL4_CPtr recv_notification, seL4_CPtr ipc_buf);
        
};

