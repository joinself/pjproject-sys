
// Copyright 2020 Self Group Ltd. All Rights Reserved.

extern crate bindgen;

use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let target = env::var("TARGET").unwrap();
    let mut defines = HashMap::<&str, &str>::new();
    let mut clang_flags = Vec::<&str>::new();

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
    } else if target == "aarch64-apple-darwin" {
        defines.insert("ARM", "1");
        defines.insert("PJ_HAS_PENTIUM", "0");
        defines.insert("PJ_IS_BIG_ENDIAN", "0");
        defines.insert("PJ_IS_LITTLE_ENDIAN", "1");
        defines.insert("PJ_M_ARM64", "1");

        clang_flags.push("-DARM");
        clang_flags.push("-DPJ_M_ARM64");
        clang_flags.push("-DPJ_HAS_PENTIUM=0");
        clang_flags.push("-DPJ_IS_BIG_ENDIAN=0");
        clang_flags.push("-DPJ_IS_LITTLE_ENDIAN=1");
    } else if target == "aarch64-apple-ios" {
        defines.insert("ARM", "1");
        defines.insert("PJ_HAS_PENTIUM", "0");
        defines.insert("PJ_IS_BIG_ENDIAN", "0");
        defines.insert("PJ_IS_LITTLE_ENDIAN", "1");
        defines.insert("PJ_M_ARM64", "1");

        clang_flags.push("-DARM");
        clang_flags.push("-DPJ_M_ARM64");
        clang_flags.push("-DPJ_HAS_PENTIUM=0");
        clang_flags.push("-DPJ_IS_BIG_ENDIAN=0");
        clang_flags.push("-DPJ_IS_LITTLE_ENDIAN=1");
    } else if target == "aarch64-apple-ios-sim" {
        defines.insert("ARM", "1");
        defines.insert("PJ_HAS_PENTIUM", "0");
        defines.insert("PJ_IS_BIG_ENDIAN", "0");
        defines.insert("PJ_IS_LITTLE_ENDIAN", "1");
        defines.insert("PJ_M_ARM64", "1");

        clang_flags.push("-DARM");
        clang_flags.push("-DPJ_M_ARM64");
        clang_flags.push("-DPJ_HAS_PENTIUM=0");
        clang_flags.push("-DPJ_IS_BIG_ENDIAN=0");
        clang_flags.push("-DPJ_IS_LITTLE_ENDIAN=1");
    } else if target == "aarch64-linux-android" {
        defines.insert("ARM", "1");
        defines.insert("PJ_HAS_PENTIUM", "0");
        defines.insert("PJ_IS_BIG_ENDIAN", "0");
        defines.insert("PJ_IS_LITTLE_ENDIAN", "1");
        defines.insert("PJ_M_ARM64", "1");

        clang_flags.push("-DARM");
        clang_flags.push("-DPJ_M_ARM64");
        clang_flags.push("-DPJ_HAS_PENTIUM=0");
        clang_flags.push("-DPJ_IS_BIG_ENDIAN=0");
        clang_flags.push("-DPJ_IS_LITTLE_ENDIAN=1");
    } else if target == "aarch64-unknown-linux-gnu" {
        defines.insert("ARM", "1");
        defines.insert("PJ_HAS_PENTIUM", "0");
        defines.insert("PJ_IS_BIG_ENDIAN", "0");
        defines.insert("PJ_IS_LITTLE_ENDIAN", "1");
        defines.insert("PJ_M_ARM64", "1");

        clang_flags.push("-DARM");
        clang_flags.push("-DPJ_M_ARM64");
        clang_flags.push("-DPJ_HAS_PENTIUM=0");
        clang_flags.push("-DPJ_IS_BIG_ENDIAN=0");
        clang_flags.push("-DPJ_IS_LITTLE_ENDIAN=1");
    }

    let mut pj_cmd = cc::Build::new();
        
    pj_cmd.warnings(false)
        .include(pjlib_includes)
        .file("vendor/pjlib/src/pj/activesock.c")
        .file("vendor/pjlib/src/pj/addr_resolv_sock.c")
        .file("vendor/pjlib/src/pj/array.c")
        .file("vendor/pjlib/src/pj/config.c")
        .file("vendor/pjlib/src/pj/ctype.c")
        .file("vendor/pjlib/src/pj/errno.c")
        .file("vendor/pjlib/src/pj/except.c")
        .file("vendor/pjlib/src/pj/fifobuf.c")
        .file("vendor/pjlib/src/pj/file_access_unistd.c")
        .file("vendor/pjlib/src/pj/file_io_ansi.c")
        .file("vendor/pjlib/src/pj/guid_simple.c")
        .file("vendor/pjlib/src/pj/hash.c")
        .file("vendor/pjlib/src/pj/ioqueue_select.c")
        .file("vendor/pjlib/src/pj/ip_helper_generic.c")
        .file("vendor/pjlib/src/pj/list.c")
        .file("vendor/pjlib/src/pj/lock.c")
        .file("vendor/pjlib/src/pj/log.c")
        .file("vendor/pjlib/src/pj/log_writer_stdout.c")
        .file("vendor/pjlib/src/pj/os_core_unix.c")
        .file("vendor/pjlib/src/pj/os_error_unix.c")
        .file("vendor/pjlib/src/pj/os_info.c")
        .file("vendor/pjlib/src/pj/os_time_common.c")
        .file("vendor/pjlib/src/pj/os_time_unix.c")
        .file("vendor/pjlib/src/pj/os_timestamp_common.c")
        .file("vendor/pjlib/src/pj/os_timestamp_posix.c")
        .file("vendor/pjlib/src/pj/pool_buf.c")
        .file("vendor/pjlib/src/pj/pool.c")
        .file("vendor/pjlib/src/pj/pool_caching.c")
        .file("vendor/pjlib/src/pj/pool_dbg.c")
        .file("vendor/pjlib/src/pj/pool_policy_malloc.c")
        .file("vendor/pjlib/src/pj/rand.c")
        .file("vendor/pjlib/src/pj/rbtree.c")
        .file("vendor/pjlib/src/pj/sock_common.c")
        .file("vendor/pjlib/src/pj/sock_qos_bsd.c")
        .file("vendor/pjlib/src/pj/sock_qos_common.c")
        .file("vendor/pjlib/src/pj/sock_select.c")
        .file("vendor/pjlib/src/pj/ssl_sock_common.c")
        .file("vendor/pjlib/src/pj/ssl_sock_darwin.c")
        .file("vendor/pjlib/src/pj/ssl_sock_dump.c")
        .file("vendor/pjlib/src/pj/ssl_sock_ossl.c")
        .file("vendor/pjlib/src/pj/string.c")
        .file("vendor/pjlib/src/pj/symbols.c")
        .file("vendor/pjlib/src/pj/timer.c")
        .file("vendor/pjlib/src/pj/types.c");
        
    for (key, value) in &defines {
        pj_cmd.define(key, *value);
    }
        
    pj_cmd.compile("pj");

    let mut pj_util_cmd = cc::Build::new();

    pj_util_cmd.warnings(false)
        .include(pjlib_includes)
        .include(pjlib_util_includes)
        .file("vendor/pjlib-util/src/pjlib-util/base64.c")
        .file("vendor/pjlib-util/src/pjlib-util/cli.c")
        .file("vendor/pjlib-util/src/pjlib-util/cli_console.c")
        .file("vendor/pjlib-util/src/pjlib-util/cli_telnet.c")
        .file("vendor/pjlib-util/src/pjlib-util/crc32.c")
        .file("vendor/pjlib-util/src/pjlib-util/dns.c")
        .file("vendor/pjlib-util/src/pjlib-util/dns_dump.c")
        .file("vendor/pjlib-util/src/pjlib-util/dns_server.c")
        .file("vendor/pjlib-util/src/pjlib-util/errno.c")
        .file("vendor/pjlib-util/src/pjlib-util/getopt.c")
        .file("vendor/pjlib-util/src/pjlib-util/hmac_md5.c")
        .file("vendor/pjlib-util/src/pjlib-util/hmac_sha1.c")
        .file("vendor/pjlib-util/src/pjlib-util/http_client.c")
        .file("vendor/pjlib-util/src/pjlib-util/json.c")
        .file("vendor/pjlib-util/src/pjlib-util/md5.c")
        .file("vendor/pjlib-util/src/pjlib-util/pcap.c")
        .file("vendor/pjlib-util/src/pjlib-util/resolver.c")
        .file("vendor/pjlib-util/src/pjlib-util/scanner.c")
        .file("vendor/pjlib-util/src/pjlib-util/sha1.c")
        .file("vendor/pjlib-util/src/pjlib-util/srv_resolver.c")
        .file("vendor/pjlib-util/src/pjlib-util/string.c")
        .file("vendor/pjlib-util/src/pjlib-util/stun_simple.c")
        .file("vendor/pjlib-util/src/pjlib-util/stun_simple_client.c")
        .file("vendor/pjlib-util/src/pjlib-util/symbols.c")
        .file("vendor/pjlib-util/src/pjlib-util/xml.c");

    for (key, value) in &defines {
        pj_util_cmd.define(key, *value);
    }

    pj_util_cmd.compile("pj-util");    

    let mut pj_nath_cmd = cc::Build::new();
    
    pj_nath_cmd.warnings(false)
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
        .file("vendor/pjnath/src/pjnath/turn_sock.c");

    for (key, value) in &defines {
        pj_nath_cmd.define(key, *value);
    }

    pj_nath_cmd.compile("pjnath");
 
    let mut pj_media_cmd = cc::Build::new();

    pj_media_cmd.warnings(false)
        .include("vendor/")
        .include(pjlib_includes)
        .include(pjlib_util_includes)
        .include(pjnath_includes)
        .include(pjmedia_includes)
        .include(pjmedia_pjmedia_includes)
        .include(srtp_includes)
        .include(srtp_crypto_includes)
        .include(speex_includes)
        .define("PJMEDIA_HAS_SPEEX_CODEC", "0")
        .define("PJMEDIA_HAS_G711_CODEC", "0")
        .define("PJMEDIA_HAS_GSM_CODEC", "0")
        .define("PJMEDIA_HAS_ILBC_CODEC", "0")
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
        .file("vendor/pjmedia/src/pjmedia/wsola.c");

    for (key, value) in &defines {
        pj_media_cmd.define(key, *value);
    }

    pj_media_cmd.compile("pjmedia");

    println!("cargo:rerun-if-changed=zrtp.h");

    // generate the bindings for pjproject headers
    let mut builder = bindgen::Builder::default();

    for value in &clang_flags {
        builder = builder.clang_arg(*value);
    }

    let bindings = builder.clang_arg("-Ivendor/pjlib/include/")
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
}