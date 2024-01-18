#include "lfqueue.h"
#define MAX_COROUTINE_NUM 128



struct runtime {
    mco_coro *cos[MAX_COROUTINE_NUM];
    mco_coro *current;
    lfqueue ready_queue;
};



struct sender {

};

struct receiver {

};

struct ipc_buffer {

};