#include "minicoro.h"

#define MAX_ITME_NUM 1024

typedef struct lfqueue_t {
	mco_coro *queue[MAX_ITME_NUM];
} lfqueue;


mco_coro* lfqueue_pop(lfqueue *q) {
	static int flag = 1;
	if (flag) {
		flag = 0;
		return q->queue[0];
	}
	return NULL;
	
}

void lfqueue_push(lfqueue *q, mco_coro *co) {
	q->queue[0] = co;
}
