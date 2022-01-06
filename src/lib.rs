#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/pjproject.rs"));

#[cfg(test)]
mod tests {
    use super::*;
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
        }
    }
}
