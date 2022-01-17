#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/pjproject.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::mem::MaybeUninit;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);

        unsafe {
            pj_rand();
            pj_init();

            let mut uninit_rtp_session: MaybeUninit<pjmedia_rtp_session> = MaybeUninit::uninit();
            pjmedia_rtp_session_init(uninit_rtp_session.as_mut_ptr(), 96, 17);

            let mut uninit_caching_pool: MaybeUninit<pj_caching_pool> = MaybeUninit::uninit();

            pj_caching_pool_init(
                uninit_caching_pool.as_mut_ptr(),
                &pj_pool_factory_default_policy,
                0,
            );

            let caching_pool = uninit_caching_pool.assume_init_mut();
            let caching_pool_name = CString::new("artp".to_owned()).unwrap().into_raw();

            let pool = pj_pool_create(
                &mut caching_pool.factory,
                caching_pool_name,
                8000,
                8000,
                None,
            );

            let sender_ssrc: u32 = 19718234;
            let default_pt = 96;
            let default_data_pt = 100;

            let mut uninit_rtp_session: MaybeUninit<pjmedia_rtp_session> = MaybeUninit::uninit();

            let mut uninit_jitter_buffer: MaybeUninit<pjmedia_jbuf> = MaybeUninit::uninit();
            let mut boxed_jitter_buffer = Box::new(uninit_jitter_buffer.as_mut_ptr());

            pjmedia_rtp_session_init(uninit_rtp_session.as_mut_ptr(), default_pt, sender_ssrc);

            let rtp_session = uninit_rtp_session.assume_init_mut();
            let jitter_buffer_name = CString::new("jbuf".to_owned()).unwrap().into_raw();

            let raw_boxed_jitter_buffer = Box::into_raw(boxed_jitter_buffer);

            pjmedia_jbuf_create(
                pool,
                &pj_str(jitter_buffer_name),
                100,
                20,
                20,
                raw_boxed_jitter_buffer,
            );

            pjmedia_jbuf_set_adaptive(*raw_boxed_jitter_buffer, 2, 1, 5);
        }
    }
}
