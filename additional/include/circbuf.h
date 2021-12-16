#include <pj/assert.h>
#include <pj/errno.h>
#include <pj/pool.h>
#include <pjmedia/frame.h>
#include <pjmedia/circbuf.h>

pj_status_t pjmedia_noinline_circ_buf_create(pj_pool_t *pool, unsigned capacity, pjmedia_circ_buf **p_cb);
pj_status_t pjmedia_noinline_circ_buf_reset(pjmedia_circ_buf *circbuf);
unsigned pjmedia_noinline_circ_buf_get_len(pjmedia_circ_buf *circbuf);
void pjmedia_noinline_circ_buf_set_len(pjmedia_circ_buf *circbuf, unsigned len);
pj_status_t pjmedia_noinline_circ_buf_adv_read_ptr(pjmedia_circ_buf *circbuf, unsigned count);
pj_status_t pjmedia_noinline_circ_buf_adv_write_ptr(pjmedia_circ_buf *circbuf, unsigned count);
void pjmedia_noinline_circ_buf_get_read_regions(pjmedia_circ_buf *circbuf, pj_int16_t **reg1, unsigned *reg1_len, pj_int16_t **reg2, unsigned *reg2_len);
void pjmedia_noinline_circ_buf_get_write_regions(pjmedia_circ_buf *circbuf, pj_int16_t **reg1, unsigned *reg1_len, pj_int16_t **reg2, unsigned *reg2_len);
pj_status_t pjmedia_noinline_circ_buf_read(pjmedia_circ_buf *circbuf, pj_int16_t *data, unsigned count);
pj_status_t pjmedia_noinline_circ_buf_write(pjmedia_circ_buf *circbuf,  pj_int16_t *data, unsigned count);
pj_status_t pjmedia_noinline_circ_buf_copy(pjmedia_circ_buf *circbuf, unsigned start_idx, pj_int16_t *data, unsigned count);
pj_status_t pjmedia_noinline_circ_buf_pack_buffer(pjmedia_circ_buf *circbuf);