#define MINICORO_IMPL
#include "scheduler.h"
#include <stdio.h>

#define seL4_CPtr int

#define MAX_RT_NUM 16

#define MAX_CONNNECTIONS 4096

static struct runtime RUNTIMES[MAX_RT_NUM];

static struct ipc_buffer *SENDER_MAP[MAX_CONNNECTIONS];


seL4_CPtr create_runtime() {
    // TODO: alloc recv ntfn, bound tcb
    // return recv ntfn
    int idle = 0;
    memset(RUNTIMES + sizeof(struct runtime) * idle, 0, sizeof(struct runtime));
    return idle;
}

void runtime_run(int runtime) {
    mco_coro *current;
    while (1) {
        while (current = lfqueue_pop(&RUNTIMES[runtime].ready_queue)) {
            RUNTIMES[runtime].current = current;
            mco_resume(current);
        }
        RUNTIMES[runtime].current = NULL;
        // suspend thread
        break;
    }
}

void coroutine_helper(mco_coro *co) {
    void (*func)(void *args);
    void *args;
    mco_pop(co, &args, sizeof(args));
    mco_pop(co, &func, sizeof(func));
    func(args);
}

int spwan_coroutine(int runtime, void (*func)(void* args), void *args) {
    mco_coro* co;
    mco_desc desc = mco_desc_init(coroutine_helper, 0);
    mco_result res = mco_create(&co, &desc);
    if(res != MCO_SUCCESS) {
        printf("Failed to create coroutine\n");
        return NULL;
    }
    mco_push(co, &func, sizeof(func));
    mco_push(co, &args, sizeof(args));
    int cid = -1;
    for (int i = 0; i < MAX_COROUTINE_NUM; ++i) {
        if (RUNTIMES[runtime].cos[i] == NULL) {
            cid = i;
            co->cid = cid;
            RUNTIMES[runtime].cos[i] = co;
            lfqueue_push(&RUNTIMES[runtime].ready_queue, co);
            break;
        }
    }
    return cid;
}

int register_connection(seL4_CPtr sender_ntfn, struct ipc_buffer *buf) {
    // todo: register sender
    SENDER_MAP[sender_ntfn] = buf;
    return 0;
}

struct args {
    int a;
    int b;
    int rt;
};

void coroutine1(void *args) {
    struct args* a = args;
    printf("a: %d, b: %d\n", a->a, a->b);
    mco_yield(RUNTIMES->current);
    printf("yield after\n");
}


int main() {
    int rt_id = create_runtime();
    

    struct args a = {
        .a = 6,
        .b = 7,
        .rt = rt_id,
    };
    mco_coro* co = spwan_coroutine(rt_id, coroutine1, &a);
    runtime_run(rt_id);
}