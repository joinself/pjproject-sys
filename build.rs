
// Copyright 2020 Self Group Ltd. All Rights Reserved.

extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let target = env::var("TARGET").unwrap();
    let pjlib_includes = Path::new("vendor/pjlib/include/");
    let pjlib_util_includes = Path::new("vendor/pjlib-util/include/");
    let pjnath_includes = Path::new("vendor/pjnath/include/");
    let pjmedia_includes = Path::new("vendor/pjmedia/include/");
    let pjmedia_pjmedia_includes = Path::new("pjmedia/include/pjmedia");
    let srtp_includes = Path::new("vendor/third_party/srtp/include/");
    let srtp_crypto_includes = Path::new("vendor/third_party/srtp/crypto/include/");
    let speex_includes = Path::new("vendor/third_party/speex/include/");

    if target == "x86_64-apple-darwin" {
        
    } else if target == "x86_64-apple-ios" {

    } else if target == "x86_64-linux-android" {

    } else if target == "x86_64-unknown-linux-gnu" {

    }

    cc::Build::new()
        .warnings(false)
        .include(pjlib_includes)
        .include(pjlib_util_includes)
        .include(pjnath_includes)
        .file("vendor/pjnath/src/pjnath/errno.c")
        .file("vendor/pjnath/src/pjnath/ice_session.c")
        .file("vendor/pjnath/src/pjnath/ice_strans.c")
        .file("vendor/pjnath/src/pjnath/nat_detect.c")
        .file("vendor/pjnath/src/pjnath/stun_auth.c")
        .file("vendor/pjnath/src/pjnath/stun_msg.c")
        .file("vendor/pjnath/src/pjnath/stun_msg_dump.c")
        .file("vendor/pjnath/src/pjnath/stun_session.c")
        .file("vendor/pjnath/src/pjnath/stun_sock.c")
        .file("vendor/pjnath/src/pjnath/stun_transaction.c")
        .file("vendor/pjnath/src/pjnath/turn_session.c")
        .file("vendor/pjnath/src/pjnath/turn_sock.c")
        .compile("pjnath");
 
    cc::Build::new()
        .warnings(false)
        .include("vendor/")
        .include(pjlib_includes)
        .include(pjlib_util_includes)
        .include(pjnath_includes)
        .include(pjmedia_includes)
        .include(pjmedia_pjmedia_includes)
        .include(srtp_includes)
        .include(srtp_crypto_includes)
        .include(speex_includes)
        // .define("PJMEDIA_EXTERNAL_SRTP", "0")
        .define("PJMEDIA_HAS_SPEEX_CODEC", "0")
        .define("PJMEDIA_HAS_G711_CODEC", "0")
        .define("PJMEDIA_HAS_GSM_CODEC", "0")
        .define("PJMEDIA_HAS_ILBC_CODEC", "0")
        .define("PJ_HAS_FLOATING_POINT", "0")
        .file("vendor/pjmedia/src/pjmedia/alaw_ulaw.c")
        .file("vendor/pjmedia/src/pjmedia/alaw_ulaw_table.c")
        .file("vendor/pjmedia/src/pjmedia/audiodev.c")
        .file("vendor/pjmedia/src/pjmedia/avi_player.c")
        .file("vendor/pjmedia/src/pjmedia/bidirectional.c")
        .file("vendor/pjmedia/src/pjmedia/clock_thread.c")
        .file("vendor/pjmedia/src/pjmedia/codec.c")
        .file("vendor/pjmedia/src/pjmedia/conference.c")
        .file("vendor/pjmedia/src/pjmedia/conf_switch.c")
        .file("vendor/pjmedia/src/pjmedia/converter.c")
        .file("vendor/pjmedia/src/pjmedia/converter_libswscale.c")
        .file("vendor/pjmedia/src/pjmedia/converter_libyuv.c")
        .file("vendor/pjmedia/src/pjmedia/delaybuf.c")
        .file("vendor/pjmedia/src/pjmedia/dummy.c")
        .file("vendor/pjmedia/src/pjmedia/echo_common.c")
        .file("vendor/pjmedia/src/pjmedia/echo_port.c")
        .file("vendor/pjmedia/src/pjmedia/echo_suppress.c")
        .file("vendor/pjmedia/src/pjmedia/echo_webrtc.c")
        .file("vendor/pjmedia/src/pjmedia/endpoint.c")
        .file("vendor/pjmedia/src/pjmedia/errno.c")
        .file("vendor/pjmedia/src/pjmedia/event.c")
        .file("vendor/pjmedia/src/pjmedia/ffmpeg_util.c")
        .file("vendor/pjmedia/src/pjmedia/format.c")
        .file("vendor/pjmedia/src/pjmedia/g711.c")
        .file("vendor/pjmedia/src/pjmedia/jbuf.c")
        .file("vendor/pjmedia/src/pjmedia/master_port.c")
        .file("vendor/pjmedia/src/pjmedia/mem_capture.c")
        .file("vendor/pjmedia/src/pjmedia/mem_player.c")
        .file("vendor/pjmedia/src/pjmedia/null_port.c")
        .file("vendor/pjmedia/src/pjmedia/plc_common.c")
        .file("vendor/pjmedia/src/pjmedia/port.c")
        .file("vendor/pjmedia/src/pjmedia/resample_libsamplerate.c")
        .file("vendor/pjmedia/src/pjmedia/resample_port.c")
        .file("vendor/pjmedia/src/pjmedia/resample_resample.c")
        .file("vendor/pjmedia/src/pjmedia/rtcp.c")
        .file("vendor/pjmedia/src/pjmedia/rtcp_fb.c")
        .file("vendor/pjmedia/src/pjmedia/rtcp_xr.c")
        .file("vendor/pjmedia/src/pjmedia/rtp.c")
        .file("vendor/pjmedia/src/pjmedia/sdp.c")
        .file("vendor/pjmedia/src/pjmedia/sdp_cmp.c")
        .file("vendor/pjmedia/src/pjmedia/sdp_neg.c")
        .file("vendor/pjmedia/src/pjmedia/session.c")
        .file("vendor/pjmedia/src/pjmedia/silencedet.c")
        .file("vendor/pjmedia/src/pjmedia/sound_legacy.c")
        .file("vendor/pjmedia/src/pjmedia/sound_port.c")
        .file("vendor/pjmedia/src/pjmedia/splitcomb.c")
        .file("vendor/pjmedia/src/pjmedia/stereo_port.c")
        .file("vendor/pjmedia/src/pjmedia/stream.c")
        .file("vendor/pjmedia/src/pjmedia/stream_common.c")
        .file("vendor/pjmedia/src/pjmedia/stream_info.c")
        .file("vendor/pjmedia/src/pjmedia/tonegen.c")
        .file("vendor/pjmedia/src/pjmedia/transport_adapter_sample.c")
        .file("vendor/pjmedia/src/pjmedia/transport_ice.c")
        .file("vendor/pjmedia/src/pjmedia/transport_loop.c")
        .file("vendor/pjmedia/src/pjmedia/transport_srtp.c")
        .file("vendor/pjmedia/src/pjmedia/transport_udp.c")
        .file("vendor/pjmedia/src/pjmedia/types.c")
        .file("vendor/pjmedia/src/pjmedia/vid_codec.c")
        .file("vendor/pjmedia/src/pjmedia/vid_codec_util.c")
        .file("vendor/pjmedia/src/pjmedia/vid_conf.c")
        .file("vendor/pjmedia/src/pjmedia/videodev.c")
        .file("vendor/pjmedia/src/pjmedia/vid_port.c")
        .file("vendor/pjmedia/src/pjmedia/vid_stream.c")
        .file("vendor/pjmedia/src/pjmedia/vid_stream_info.c")
        .file("vendor/pjmedia/src/pjmedia/vid_tee.c")
        .file("vendor/pjmedia/src/pjmedia/wave.c")
        .file("vendor/pjmedia/src/pjmedia/wav_player.c")
        .file("vendor/pjmedia/src/pjmedia/wav_playlist.c")
        .file("vendor/pjmedia/src/pjmedia/wav_writer.c")
        .file("vendor/pjmedia/src/pjmedia/wsola.c")
        .compile("pjmedia");

    println!("cargo:rerun-if-changed=zrtp.h");

    // generate the bindings for pjproject headers
    let bindings = bindgen::Builder::default()
        .clang_arg("-Ivendor/pjlib/include/")
        .clang_arg("-Ivendor/pjlib-util/include/")
        .clang_arg("-Ivendor/pjnath/include/")
        .clang_arg("-Ivendor/pjmedia/include/")
        .clang_arg("-Ivendor/pjsip/include/")
        .allowlist_type(r"pj.*")
        .allowlist_type(r"PJ.*")
        .allowlist_function(r"pj.*")
        .allowlist_function(r"PJ.*")
        .allowlist_var(r"pj.*")
        .allowlist_var(r"PJ.*")
        .header("pjproject.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate pjproject bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // output the bindings
    bindings
        .write_to_file(out_path.join("pjproject.rs"))
        .expect("Couldn't write pjproject bindings!");


    // link generated libraries
    
    println!("cargo:rustc-link-search=vendor/pjlib/lib/");
    println!("cargo:rustc-link-lib=pj-{}", target);

    println!("cargo:rustc-link-search=vendor/pjlib-util/lib/");
    println!("cargo:rustc-link-lib=pjlib-util-{}", target);

    println!("cargo:rustc-link-search=vendor/pjmedia/lib/");
    println!("cargo:rustc-link-lib=pjmedia-{}", target);
    println!("cargo:rustc-link-lib=pjmedia-audiodev-{}", target);
    println!("cargo:rustc-link-lib=pjmedia-codec-{}", target);
    println!("cargo:rustc-link-lib=pjmedia-videodev-{}", target);
    println!("cargo:rustc-link-lib=pjsdp-{}", target);

    println!("cargo:rustc-link-search=vendor/pjnath/lib/");
    println!("cargo:rustc-link-lib=pjnath-{}", target);

    println!("cargo:rustc-link-search=vendor/pjsip/lib/");
    println!("cargo:rustc-link-lib=pjsip-{}", target);
    println!("cargo:rustc-link-lib=pjsip-ua-{}", target);
    println!("cargo:rustc-link-lib=pjsip-simple-{}", target);
    println!("cargo:rustc-link-lib=pjsua-{}", target);
    println!("cargo:rustc-link-lib=pjsua2-{}", target);

    println!("cargo:rustc-link-lib=uuid");
}