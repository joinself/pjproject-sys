#include <pj/types.h>

int pj_noinline_cmp_timestamp(const pj_timestamp *t1, const pj_timestamp *t2)
{
    // We're only targeting 64 bit systems, but check if wasm supports 64 bit ints
    if (t1->u64 < t2->u64)
	return -1;
    else if (t1->u64 > t2->u64)
	return 1;
    else
	return 0;
}