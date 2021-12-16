#include <pj/assert.h>
#include <pj/errno.h>
#include <pj/pool.h>
#include <pjmedia/frame.h>
#include <pjmedia/circbuf.h>

/**
 * Create the circular buffer.
 *
 * @param pool		    Pool where the circular buffer will be allocated
 *			    from.
 * @param capacity	    Capacity of the buffer, in samples.
 * @param p_cb		    Pointer to receive the circular buffer instance.
 *
 * @return		    PJ_SUCCESS if the circular buffer has been
 *			    created successfully, otherwise the appropriate
 *			    error will be returned.
 */
pj_status_t pjmedia_noinline_circ_buf_create(pj_pool_t *pool, 
					       unsigned capacity, 
					       pjmedia_circ_buf **p_cb)
{
    pjmedia_circ_buf *cbuf;

    cbuf = PJ_POOL_ZALLOC_T(pool, pjmedia_circ_buf);
    cbuf->buf = (pj_int16_t*) pj_pool_calloc(pool, capacity, 
					     sizeof(pj_int16_t));
    cbuf->capacity = capacity;
    cbuf->start = cbuf->buf;
    cbuf->len = 0;

    *p_cb = cbuf;

    return PJ_SUCCESS;
}


/**
 * Reset the circular buffer.
 *
 * @param circbuf	    The circular buffer.
 *
 * @return		    PJ_SUCCESS when successful.
 */
pj_status_t pjmedia_noinline_circ_buf_reset(pjmedia_circ_buf *circbuf)
{
    circbuf->start = circbuf->buf;
    circbuf->len = 0;

    return PJ_SUCCESS;
}


/**
 * Get the circular buffer length, it is number of samples buffered in the 
 * circular buffer.
 *
 * @param circbuf	    The circular buffer.
 *
 * @return		    The buffer length.
 */
unsigned pjmedia_noinline_circ_buf_get_len(pjmedia_circ_buf *circbuf)
{
    return circbuf->len;
}


/**
 * Set circular buffer length. This is useful when audio buffer is manually 
 * manipulated by the user, e.g: shrinked, expanded.
 *
 * @param circbuf	    The circular buffer.
 * @param len		    The new buffer length.
 */
void pjmedia_noinline_circ_buf_set_len(pjmedia_circ_buf *circbuf,
					 unsigned len)
{
    PJMEDIA_CIRC_BUF_CHECK(len <= circbuf->capacity);
    circbuf->len = len;
}


/**
 * Advance the read pointer of circular buffer. This function will discard
 * the skipped samples while advancing the read pointer, thus reducing 
 * the buffer length.
 *
 * @param circbuf	    The circular buffer.
 * @param count		    Distance from current read pointer, can only be
 *			    possitive number, in samples.
 *
 * @return		    PJ_SUCCESS when successful, otherwise 
 *			    the appropriate error will be returned.
 */
pj_status_t pjmedia_noinline_circ_buf_adv_read_ptr(pjmedia_circ_buf *circbuf, 
						     unsigned count)
{
    if (count >= circbuf->len)
	return pjmedia_circ_buf_reset(circbuf);

    PJMEDIA_CIRC_BUF_CHECK(count <= circbuf->len);

    circbuf->start += count;
    if (circbuf->start >= circbuf->buf + circbuf->capacity) 
	circbuf->start -= circbuf->capacity;
    circbuf->len -= count;

    return PJ_SUCCESS;
}


/**
 * Advance the write pointer of circular buffer. Since write pointer is always
 * pointing to a sample after the end of sample, so this function also means
 * increasing the buffer length.
 *
 * @param circbuf	    The circular buffer.
 * @param count		    Distance from current write pointer, can only be
 *			    possitive number, in samples.
 *
 * @return		    PJ_SUCCESS when successful, otherwise 
 *			    the appropriate error will be returned.
 */
pj_status_t pjmedia_noinline_circ_buf_adv_write_ptr(pjmedia_circ_buf *circbuf,
						      unsigned count)
{
    if (count + circbuf->len > circbuf->capacity)
	return PJ_ETOOBIG;

    circbuf->len += count;

    return PJ_SUCCESS;
}


/**
 * Get the real buffer addresses containing the audio samples.
 *
 * @param circbuf	    The circular buffer.
 * @param reg1		    Pointer to store the first buffer address.
 * @param reg1_len	    Pointer to store the length of the first buffer, 
 *			    in samples.
 * @param reg2		    Pointer to store the second buffer address.
 * @param reg2_len	    Pointer to store the length of the second buffer, 
 *			    in samples.
 */
void pjmedia_noinline_circ_buf_get_read_regions(pjmedia_circ_buf *circbuf, 
						  pj_int16_t **reg1, 
						  unsigned *reg1_len, 
						  pj_int16_t **reg2, 
						  unsigned *reg2_len)
{
    *reg1 = circbuf->start;
    *reg1_len = circbuf->len;
    if (*reg1 + *reg1_len > circbuf->buf + circbuf->capacity) {
	*reg1_len = (unsigned)(circbuf->buf + circbuf->capacity - 
			       circbuf->start);
	*reg2 = circbuf->buf;
	*reg2_len = circbuf->len - *reg1_len;
    } else {
	*reg2 = NULL;
	*reg2_len = 0;
    }

    PJMEDIA_CIRC_BUF_CHECK(*reg1_len != 0 || (*reg1_len == 0 && 
					      circbuf->len == 0));
    PJMEDIA_CIRC_BUF_CHECK(*reg1_len + *reg2_len == circbuf->len);
}


/**
 * Get the real buffer addresses that is empty or writeable.
 *
 * @param circbuf	    The circular buffer.
 * @param reg1		    Pointer to store the first buffer address.
 * @param reg1_len	    Pointer to store the length of the first buffer, 
 *			    in samples.
 * @param reg2		    Pointer to store the second buffer address.
 * @param reg2_len	    Pointer to store the length of the second buffer, 
 *			    in samples.
 */
void pjmedia_noinline_circ_buf_get_write_regions(pjmedia_circ_buf *circbuf, 
						   pj_int16_t **reg1, 
						   unsigned *reg1_len, 
						   pj_int16_t **reg2, 
						   unsigned *reg2_len)
{
    *reg1 = circbuf->start + circbuf->len;
    if (*reg1 >= circbuf->buf + circbuf->capacity)
	*reg1 -= circbuf->capacity;
    *reg1_len = circbuf->capacity - circbuf->len;
    if (*reg1 + *reg1_len > circbuf->buf + circbuf->capacity) {
	*reg1_len = (unsigned)(circbuf->buf + circbuf->capacity - *reg1);
	*reg2 = circbuf->buf;
	*reg2_len = (unsigned)(circbuf->start - circbuf->buf);
    } else {
	*reg2 = NULL;
	*reg2_len = 0;
    }

    PJMEDIA_CIRC_BUF_CHECK(*reg1_len != 0 || (*reg1_len == 0 && 
					      circbuf->len == 0));
    PJMEDIA_CIRC_BUF_CHECK(*reg1_len + *reg2_len == circbuf->capacity - 
			   circbuf->len);
}


/**
 * Read audio samples from the circular buffer.
 *
 * @param circbuf	    The circular buffer.
 * @param data		    Buffer to store the read audio samples.
 * @param count		    Number of samples being read.
 *
 * @return		    PJ_SUCCESS when successful, otherwise 
 *			    the appropriate error will be returned.
 */
pj_status_t pjmedia_noinline_circ_buf_read(pjmedia_circ_buf *circbuf, 
					     pj_int16_t *data, 
					     unsigned count)
{
    pj_int16_t *reg1, *reg2;
    unsigned reg1cnt, reg2cnt;

    /* Data in the buffer is less than requested */
    if (count > circbuf->len)
	return PJ_ETOOBIG;

    pjmedia_circ_buf_get_read_regions(circbuf, &reg1, &reg1cnt, 
				      &reg2, &reg2cnt);
    if (reg1cnt >= count) {
	pjmedia_copy_samples(data, reg1, count);
    } else {
	pjmedia_copy_samples(data, reg1, reg1cnt);
	pjmedia_copy_samples(data + reg1cnt, reg2, count - reg1cnt);
    }

    return pjmedia_circ_buf_adv_read_ptr(circbuf, count);
}


/**
 * Write audio samples to the circular buffer.
 *
 * @param circbuf	    The circular buffer.
 * @param data		    Audio samples to be written.
 * @param count		    Number of samples being written.
 *
 * @return		    PJ_SUCCESS when successful, otherwise
 *			    the appropriate error will be returned.
 */
pj_status_t pjmedia_noinline_circ_buf_write(pjmedia_circ_buf *circbuf, 
					      pj_int16_t *data, 
					      unsigned count)
{
    pj_int16_t *reg1, *reg2;
    unsigned reg1cnt, reg2cnt;

    /* Data to write is larger than buffer can store */
    if (count > circbuf->capacity - circbuf->len)
	return PJ_ETOOBIG;

    pjmedia_circ_buf_get_write_regions(circbuf, &reg1, &reg1cnt, 
				       &reg2, &reg2cnt);
    if (reg1cnt >= count) {
	pjmedia_copy_samples(reg1, data, count);
    } else {
	pjmedia_copy_samples(reg1, data, reg1cnt);
	pjmedia_copy_samples(reg2, data + reg1cnt, count - reg1cnt);
    }

    return pjmedia_circ_buf_adv_write_ptr(circbuf, count);
}


/**
 * Copy audio samples from the circular buffer without changing its state. 
 *
 * @param circbuf	    The circular buffer.
 * @param start_idx	    Starting sample index to be copied.
 * @param data		    Buffer to store the read audio samples.
 * @param count		    Number of samples being read.
 *
 * @return		    PJ_SUCCESS when successful, otherwise 
 *			    the appropriate error will be returned.
 */
pj_status_t pjmedia_noinline_circ_buf_copy(pjmedia_circ_buf *circbuf, 
					     unsigned start_idx,
					     pj_int16_t *data, 
					     unsigned count)
{
    pj_int16_t *reg1, *reg2;
    unsigned reg1cnt, reg2cnt;

    /* Data in the buffer is less than requested */
    if (count + start_idx > circbuf->len)
	return PJ_ETOOBIG;

    pjmedia_circ_buf_get_read_regions(circbuf, &reg1, &reg1cnt, 
				      &reg2, &reg2cnt);
    if (reg1cnt > start_idx) {
	unsigned tmp_len;
	tmp_len = reg1cnt - start_idx;
	if (tmp_len > count)
	    tmp_len = count;
	pjmedia_copy_samples(data, reg1 + start_idx, tmp_len);
	if (tmp_len < count)
	    pjmedia_copy_samples(data + tmp_len, reg2, count - tmp_len);
    } else {
	pjmedia_copy_samples(data, reg2 + start_idx - reg1cnt, count);
    }

    return PJ_SUCCESS;
}


/**
 * Pack the buffer so the first sample will be in the beginning of the buffer.
 * This will also make the buffer contiguous.
 *
 * @param circbuf	    The circular buffer.
 *
 * @return		    PJ_SUCCESS when successful, otherwise 
 *			    the appropriate error will be returned.
 */
pj_status_t pjmedia_noinline_circ_buf_pack_buffer(pjmedia_circ_buf *circbuf)
{
    pj_int16_t *reg1, *reg2;
    unsigned reg1cnt, reg2cnt;
    unsigned gap;

    pjmedia_circ_buf_get_read_regions(circbuf, &reg1, &reg1cnt, 
				      &reg2, &reg2cnt);

    /* Check if not contigue */
    if (reg2cnt != 0) {
	/* Check if no space left to roll the buffer 
	 * (or should this function provide temporary buffer?)
	 */
	gap = circbuf->capacity - pjmedia_circ_buf_get_len(circbuf);
	if (gap == 0)
	    return PJ_ETOOBIG;

	/* Roll buffer left using the gap until reg2cnt == 0 */
	do {
	    if (gap > reg2cnt)
		gap = reg2cnt;
	    pjmedia_move_samples(reg1 - gap, reg1, reg1cnt);
	    pjmedia_copy_samples(reg1 + reg1cnt - gap, reg2, gap);
	    if (gap < reg2cnt)
		pjmedia_move_samples(reg2, reg2 + gap, reg2cnt - gap);
	    reg1 -= gap;
	    reg1cnt += gap;
	    reg2cnt -= gap;
	} while (reg2cnt > 0);
    }

    /* Finally, Shift samples to the left edge */
    if (reg1 != circbuf->buf)
	pjmedia_move_samples(circbuf->buf, reg1, 
			     pjmedia_circ_buf_get_len(circbuf));
    circbuf->start = circbuf->buf;

    return PJ_SUCCESS;
}