#include <pjnath/stun_msg.h>
#include <pjnath/stun_config.h>
#include <pj/assert.h>
#include <pj/errno.h>
#include <pj/string.h>

void pj_noinline_stun_config_init(pj_stun_config *cfg,
				    pj_pool_factory *factory,
				    unsigned options,
				    pj_ioqueue_t *ioqueue,
				    pj_timer_heap_t *timer_heap)
{
    pj_bzero(cfg, sizeof(*cfg));

    cfg->pf = factory;
    cfg->options = options;
    cfg->ioqueue = ioqueue;
    cfg->timer_heap = timer_heap;
    cfg->rto_msec = PJ_STUN_RTO_VALUE;
    cfg->res_cache_msec = PJ_STUN_RES_CACHE_DURATION;
    cfg->software_name = pj_str((char*)PJNATH_STUN_SOFTWARE_NAME);
}