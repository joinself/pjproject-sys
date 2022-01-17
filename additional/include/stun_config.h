#include <pjnath/stun_msg.h>
#include <pjnath/stun_config.h>
#include <pj/assert.h>
#include <pj/errno.h>
#include <pj/string.h>

void pj_noinline_stun_config_init(pj_stun_config *cfg, pj_pool_factory *factory, unsigned options, pj_ioqueue_t *ioqueue, pj_timer_heap_t *timer_heap);