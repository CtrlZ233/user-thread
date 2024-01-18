#define MINICORO_IMPL
#include "minicoro.h"
#include <stdio.h>


struct args {
    int a;
    int b;
};

void coroutine1(void *args) {
    struct args* a = args;
    printf("a: %d, b: %d\n", a->a, a->b);
}

void coroutine_helper(mco_coro* co) {
    void (*func)(void *args);
    void *args;
    mco_pop(co, &args, sizeof(args));
    mco_pop(co, &func, sizeof(func));
    func(args);
}

mco_coro* spwan_coroutine(int runtime, void (*func)(void *args), void *args) {
    mco_coro* co;
    mco_desc desc = mco_desc_init(coroutine_helper, 0);
    mco_result res = mco_create(&co, &desc);
    if(res != MCO_SUCCESS) {
        printf("Failed to create coroutine\n");
        return NULL;
    }
    mco_push(co, &func, sizeof(func));
    mco_push(co, &args, sizeof(args));
    // mco_resume(co);
    return co;
}


int main() {
    struct args a = {
        .a = 6,
        .b = 7,
    };
    mco_coro* co = spwan_coroutine(0, coroutine1, &a);
    mco_resume(co);
}