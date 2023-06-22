// Copyright 2020 Self Group Ltd. All Rights Reserved.

extern crate bindgen;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

fn main() {
    let target = env::var("TARGET").unwrap();
    let mut defines = HashMap::<&str, &str>::new();
    let mut clang_flags = Vec::<String>::new();

    let pjlib_includes = Path::new("vendor/pjlib/include/");
    let pjlib_util_includes = Path::new("vendor/pjlib-util/include/");
    let pjnath_includes = Path::new("vendor/pjnath/include/");
    let pjmedia_includes = Path::new("vendor/pjmedia/include/");
    let pjmedia_pjmedia_includes = Path::new("pjmedia/include/pjmedia");
    let srtp_includes = Path::new("vendor/third_party/srtp/include/");
    let srtp_crypto_includes = Path::new("vendor/third_party/srtp/crypto/include/");
    let speex_includes = Path::new("vendor/third_party/speex/include/");
    let additional_includes = Path::new("additional/include/");

    if target == "i686-linux-android" {
        defines.insert("PJ_ANDROID", "1");
        defines.insert("ARM", "0");
        defines.insert("PJ_HAS_PENTIUM", "1");
        defines.insert("PJ_IS_BIG_ENDIAN", "0");
        defines.insert("PJ_IS_LITTLE_ENDIAN", "1");
        defines.insert("PJ_M_I386", "1");
        defines.insert("PJ_JNI_HAS_JNI_ONLOAD", "0");
        defines.insert("PJ_TIMER_DEBUG", "0");

        // TODO : replace this hack with something else
        let define_from = "#define PJ_HAS_SYS_TIMEB_H	    1";
        let define_to = "#define PJ_HAS_SYS_TIMEB_H	    0";

        let os_linux = Path::new("./vendor/pjlib/include/pj/compat/os_linux.h");
        let mut src = File::open(os_linux).unwrap();

        let mut header_data = String::new();
        src.read_to_string(&mut header_data).unwrap();
        drop(src); // Close the file early

        // Run the replace operation in memory
        let new_header_data = header_data.replace(define_from, define_to);

        // Recreate the file and dump the processed contents to it
        let mut dst = File::create(os_linux).unwrap();
        dst.write_all(new_header_data.as_bytes()).unwrap();

        clang_flags.push(String::from("-DPJ_M_I386"));
        clang_flags.push(String::from("-DPJ_HAS_PENTIUM=1"));
        clang_flags.push(String::from("-DPJ_IS_BIG_ENDIAN=0"));
        clang_flags.push(String::from("-DPJ_IS_LITTLE_ENDIAN=1"));
        clang_flags.push(String::from("-DPJ_TIMER_DEBUG=0"));
    } else if target == "x86_64-apple-darwin" || target == "x86_64-apple-ios" {
    } else if target == "x86_64-linux-android" {
        defines.insert("PJ_ANDROID", "1");
        defines.insert("ARM", "0");
        defines.insert("PJ_HAS_PENTIUM", "1");
        defines.insert("PJ_IS_BIG_ENDIAN", "0");
        defines.insert("PJ_IS_LITTLE_ENDIAN", "1");
        defines.insert("PJ_M_X86_64", "1");
        defines.insert("PJ_JNI_HAS_JNI_ONLOAD", "0");
        defines.insert("PJ_TIMER_DEBUG", "0");

        // TODO : replace this hack with something else
        let define_from = "#define PJ_HAS_SYS_TIMEB_H	    1";
        let define_to = "#define PJ_HAS_SYS_TIMEB_H	    0";

        let os_linux = Path::new("./vendor/pjlib/include/pj/compat/os_linux.h");
        let mut src = File::open(os_linux).unwrap();

        let mut header_data = String::new();
        src.read_to_string(&mut header_data).unwrap();
        drop(src); // Close the file early

        // Run the replace operation in memory
        let new_header_data = header_data.replace(define_from, define_to);

        // Recreate the file and dump the processed contents to it
        let mut dst = File::create(os_linux).unwrap();
        dst.write_all(new_header_data.as_bytes()).unwrap();

        clang_flags.push(String::from("-DPJ_M_X86_64"));
        clang_flags.push(String::from("-DPJ_HAS_PENTIUM=1"));
        clang_flags.push(String::from("-DPJ_IS_BIG_ENDIAN=0"));
        clang_flags.push(String::from("-DPJ_IS_LITTLE_ENDIAN=1"));
        clang_flags.push(String::from("-DPJ_TIMER_DEBUG=0"));
    } else if target == "x86_64-unknown-linux-gnu" {
        defines.insert("PJ_LINUX", "1");
        defines.insert("PJ_SOCK_HAS_INET_ATON", "1");
        defines.insert("PJ_SOCK_HAS_INET_PTON", "1");
        defines.insert("PJ_SOCK_HAS_INET_NTOP", "1");
        defines.insert("PJ_SOCK_HAS_GETADDRINFO", "1");
        defines.insert("PJ_HAS_ARPA_INET_H", "1");
        defines.insert("PJ_HAS_ASSERT_H", "1");
        defines.insert("PJ_HAS_CTYPE_H", "1");
        defines.insert("PJ_HAS_ERRNO_H", "1");
        defines.insert("PJ_HAS_FCNTL_H", "1");
        defines.insert("PJ_HAS_LIMITS_H", "1");
        defines.insert("PJ_HAS_MALLOC_H", "1");
        defines.insert("PJ_HAS_NETDB_H", "1");
        defines.insert("PJ_HAS_NETINET_IN_SYSTM_H", "1");
        defines.insert("PJ_HAS_NETINET_IN_H", "1");
        defines.insert("PJ_HAS_NETINET_IP_H", "1");
        defines.insert("PJ_HAS_NETINET_TCP_H", "1");
        defines.insert("PJ_HAS_NET_IF_H", "1");
        defines.insert("PJ_HAS_IFADDRS_H", "1");
        defines.insert("PJ_HAS_SEMAPHORE_H", "1");
        defines.insert("PJ_HAS_SETJMP_H", "1");
        defines.insert("PJ_HAS_STDARG_H", "1");
        defines.insert("PJ_HAS_STDDEF_H", "1");
        defines.insert("PJ_HAS_STDIO_H", "1");
        defines.insert("PJ_HAS_STDINT_H", "1");
        defines.insert("PJ_HAS_STDLIB_H", "1");
        defines.insert("PJ_HAS_STRING_H", "1");
        defines.insert("PJ_HAS_SYS_IOCTL_H", "1");
        defines.insert("PJ_HAS_SYS_SELECT_H", "1");
        defines.insert("PJ_HAS_SYS_SOCKET_H", "1");
        defines.insert("PJ_HAS_SYS_TIME_H", "1");
        defines.insert("PJ_HAS_SYS_TIMEB_H", "1");
        defines.insert("PJ_HAS_SYS_TYPES_H", "1");
        defines.insert("PJ_HAS_SYS_UTSNAME_H", "1");
        defines.insert("PJ_HAS_TIME_H", "1");
        defines.insert("PJ_HAS_UNISTD_H", "1");
        defines.insert("PJ_HAS_SEMAPHORE", "1");
        defines.insert("PJ_HAS_SOCKLEN_T", "1");
        defines.insert("PJ_SELECT_NEEDS_NFDS", "0");
        defines.insert("PJ_HAS_ERRNO_VAR", "1");
        defines.insert("PJ_HAS_SO_ERROR", "1");
        defines.insert("PJ_BLOCKING_ERROR_VAL", "EAGAIN");
        defines.insert("PJ_BLOCKING_CONNECT_ERROR_VAL", "EINPROGRESS");
        defines.insert("PJ_HAS_HIGH_RES_TIMER", "1");
        defines.insert("PJ_HAS_MALLOC", "1");
        defines.insert("PJ_NATIVE_STRING_IS_UNICODE", "0");
        defines.insert("PJ_POOL_ALIGNMENT", "8");
        defines.insert("PJ_ATOMIC_VALUE_TYPE", "long");
        defines.insert("PJ_EMULATE_RWMUTEX", "0");
        defines.insert("PJ_THREAD_SET_STACK_SIZE", "0");
        defines.insert("PJ_THREAD_ALLOCATE_STACK", "0");
        defines.insert("PJ_TIMER_DEBUG", "0");

        clang_flags.push(String::from("-DPJ_TIMER_DEBUG=0"));
    } else if target == "armv7-linux-androideabi" {
        defines.insert("PJ_ANDROID", "1");
        defines.insert("ARM", "1");
        defines.insert("PJ_HAS_PENTIUM", "0");
        defines.insert("PJ_IS_BIG_ENDIAN", "0");
        defines.insert("PJ_IS_LITTLE_ENDIAN", "1");
        defines.insert("PJ_M_ARMV7", "1");
        defines.insert("PJ_JNI_HAS_JNI_ONLOAD", "0");
        defines.insert("PJ_TIMER_DEBUG", "0");

        // TODO : replace this hack with something else
        let define_from = "#define PJ_HAS_SYS_TIMEB_H	    1";
        let define_to = "#define PJ_HAS_SYS_TIMEB_H	    0";

        let os_linux = Path::new("./vendor/pjlib/include/pj/compat/os_linux.h");
        let mut src = File::open(os_linux).unwrap();

        let mut header_data = String::new();
        src.read_to_string(&mut header_data).unwrap();
        drop(src); // Close the file early

        // Run the replace operation in memory
        let new_header_data = header_data.replace(define_from, define_to);

        // Recreate the file and dump the processed contents to it
        let mut dst = File::create(os_linux).unwrap();
        dst.write_all(new_header_data.as_bytes()).unwrap();

        clang_flags.push(String::from("-DARM"));
        clang_flags.push(String::from("-DPJ_M_ARMV7"));
        clang_flags.push(String::from("-DPJ_HAS_PENTIUM=0"));
        clang_flags.push(String::from("-DPJ_IS_BIG_ENDIAN=0"));
        clang_flags.push(String::from("-DPJ_IS_LITTLE_ENDIAN=1"));
        clang_flags.push(String::from("-DPJ_TIMER_DEBUG=0"));
    } else if target == "aarch64-apple-darwin" {
        defines.insert("ARM", "1");
        defines.insert("PJ_HAS_PENTIUM", "0");
        defines.insert("PJ_IS_BIG_ENDIAN", "0");
        defines.insert("PJ_IS_LITTLE_ENDIAN", "1");
        defines.insert("PJ_M_ARM64", "1");

        clang_flags.push(String::from("-DARM"));
        clang_flags.push(String::from("-DPJ_M_ARM64"));
        clang_flags.push(String::from("-DPJ_HAS_PENTIUM=0"));
        clang_flags.push(String::from("-DPJ_IS_BIG_ENDIAN=0"));
        clang_flags.push(String::from("-DPJ_IS_LITTLE_ENDIAN=1"));
    } else if target == "aarch64-apple-ios" || target == "aarch64-apple-ios-sim" {
        defines.insert("ARM", "1");
        defines.insert("PJ_HAS_PENTIUM", "0");
        defines.insert("PJ_IS_BIG_ENDIAN", "0");
        defines.insert("PJ_IS_LITTLE_ENDIAN", "1");
        defines.insert("PJ_M_ARM64", "1");

        defines.insert("PJ_DARWINOS", "1");
        defines.insert("PJ_HAS_ARPA_INET_H", "1");
        defines.insert("PJ_HAS_ASSERT_H", "1");
        defines.insert("PJ_HAS_CTYPE_H", "1");
        defines.insert("PJ_HAS_ERRNO_H", "1");
        defines.insert("PJ_HAS_FCNTL_H", "1");
        defines.insert("PJ_HAS_LIMITS_H", "1");
        defines.insert("PJ_HAS_NETDB_H", "1");
        defines.insert("PJ_HAS_NETINET_IN_SYSTM_H", "1");
        defines.insert("PJ_HAS_NETINET_IN_H", "1");
        defines.insert("PJ_HAS_NETINET_IP_H", "1");
        defines.insert("PJ_HAS_NETINET_TCP_H", "1");
        defines.insert("PJ_HAS_NET_IF_H", "1");
        defines.insert("PJ_HAS_IFADDRS_H", "1");
        defines.insert("PJ_HAS_SEMAPHORE_H", "1");
        defines.insert("PJ_HAS_SETJMP_H", "1");
        defines.insert("PJ_HAS_STDARG_H", "1");
        defines.insert("PJ_HAS_STDDEF_H", "1");
        defines.insert("PJ_HAS_STDIO_H", "1");
        defines.insert("PJ_HAS_STDINT_H", "1");
        defines.insert("PJ_HAS_STDLIB_H", "1");
        defines.insert("PJ_HAS_STRING_H", "1");
        defines.insert("PJ_HAS_SYS_IOCTL_H", "1");
        defines.insert("PJ_HAS_SYS_SELECT_H", "1");
        defines.insert("PJ_HAS_SYS_SOCKET_H", "1");
        defines.insert("PJ_HAS_SYS_TIME_H", "1");
        defines.insert("PJ_HAS_SYS_TIMEB_H", "1");
        defines.insert("PJ_HAS_SYS_TYPES_H", "1");
        defines.insert("PJ_HAS_SYS_FILIO_H", "1");
        defines.insert("PJ_HAS_SYS_SOCKIO_H", "1");
        defines.insert("PJ_HAS_SYS_UTSNAME_H", "1");
        defines.insert("PJ_HAS_TIME_H", "1");
        defines.insert("PJ_HAS_UNISTD_H", "1");
        defines.insert("PJ_SOCK_HAS_IPV6_V6ONLY", "1");
        defines.insert("PJ_SOCK_HAS_INET_ATON", "1");
        defines.insert("PJ_SOCK_HAS_INET_PTON", "1");
        defines.insert("PJ_SOCK_HAS_INET_NTOP", "1");
        defines.insert("PJ_SOCK_HAS_GETADDRINFO", "1");
        defines.insert("PJ_HAS_SEMAPHORE", "1");
        defines.insert("PJ_SOCKADDR_HAS_LEN", "1");
        defines.insert("PJ_HAS_SOCKLEN_T", "1");
        defines.insert("PJ_SELECT_NEEDS_NFDS", "0");
        defines.insert("PJ_HAS_ERRNO_VAR", "1");
        defines.insert("PJ_HAS_SO_ERROR", "1");
        // defines.insert("PJ_BLOCKING_ERROR_VAL", "EAGAIN");
        defines.insert("PJ_BLOCKING_CONNECT_ERROR_VAL", "EINPROGRESS");
        defines.insert("PJ_HAS_HIGH_RES_TIMER", "1");
        defines.insert("PJ_HAS_MALLOC", "1");
        defines.insert("PJ_HAS_LOCALTIME_R", "1");
        defines.insert("PJ_NATIVE_STRING_IS_UNICODE", "0");
        defines.insert("PJ_POOL_ALIGNMENT", "8");
        defines.insert("PJ_ATOMIC_VALUE_TYPE", "long");

        clang_flags.push(String::from("-DARM"));
        clang_flags.push(String::from("-DPJ_M_ARM64"));
        clang_flags.push(String::from("-DPJ_HAS_PENTIUM=0"));
        clang_flags.push(String::from("-DPJ_IS_BIG_ENDIAN=0"));
        clang_flags.push(String::from("-DPJ_IS_LITTLE_ENDIAN=1"));
    } else if target == "aarch64-linux-android" {
        defines.insert("PJ_ANDROID", "1");
        defines.insert("ARM", "1");
        defines.insert("PJ_HAS_PENTIUM", "0");
        defines.insert("PJ_IS_BIG_ENDIAN", "0");
        defines.insert("PJ_IS_LITTLE_ENDIAN", "1");
        defines.insert("PJ_M_ARM64", "1");
        defines.insert("PJ_JNI_HAS_JNI_ONLOAD", "0");
        defines.insert("PJ_TIMER_DEBUG", "0");
        defines.insert("PJ_SOCK_HAS_INET_ATON", "1");
        defines.insert("PJ_SOCK_HAS_INET_PTON", "1");
        defines.insert("PJ_SOCK_HAS_INET_NTOP", "1");
        defines.insert("PJ_SOCK_HAS_GETADDRINFO", "1");
        defines.insert("PJ_HAS_ARPA_INET_H", "1");
        defines.insert("PJ_HAS_ASSERT_H", "1");
        defines.insert("PJ_HAS_CTYPE_H", "1");
        defines.insert("PJ_HAS_ERRNO_H", "1");
        defines.insert("PJ_HAS_FCNTL_H", "1");
        defines.insert("PJ_HAS_LIMITS_H", "1");
        defines.insert("PJ_HAS_MALLOC_H", "1");
        defines.insert("PJ_HAS_NETDB_H", "1");
        defines.insert("PJ_HAS_NETINET_IN_SYSTM_H", "1");
        defines.insert("PJ_HAS_NETINET_IN_H", "1");
        defines.insert("PJ_HAS_NETINET_IP_H", "1");
        defines.insert("PJ_HAS_NETINET_TCP_H", "1");
        defines.insert("PJ_HAS_NET_IF_H", "1");
        defines.insert("PJ_HAS_IFADDRS_H", "1");
        defines.insert("PJ_HAS_SEMAPHORE_H", "1");
        defines.insert("PJ_HAS_SETJMP_H", "1");
        defines.insert("PJ_HAS_STDARG_H", "1");
        defines.insert("PJ_HAS_STDDEF_H", "1");
        defines.insert("PJ_HAS_STDIO_H", "1");
        defines.insert("PJ_HAS_STDINT_H", "1");
        defines.insert("PJ_HAS_STDLIB_H", "1");
        defines.insert("PJ_HAS_STRING_H", "1");
        defines.insert("PJ_HAS_SYS_IOCTL_H", "1");
        defines.insert("PJ_HAS_SYS_SELECT_H", "1");
        defines.insert("PJ_HAS_SYS_SOCKET_H", "1");
        defines.insert("PJ_HAS_SYS_TIME_H", "1");
        defines.insert("PJ_HAS_SYS_TYPES_H", "1");
        defines.insert("PJ_HAS_SYS_UTSNAME_H", "1");
        defines.insert("PJ_HAS_TIME_H", "1");
        defines.insert("PJ_HAS_UNISTD_H", "1");
        defines.insert("PJ_HAS_SEMAPHORE", "1");

        defines.insert("PJ_HAS_SOCKLEN_T", "1");
        defines.insert("PJ_SELECT_NEEDS_NFDS", "0");
        defines.insert("PJ_HAS_ERRNO_VAR", "1");
        defines.insert("PJ_HAS_SO_ERROR", "1");
        defines.insert("PJ_BLOCKING_ERROR_VAL", "EAGAIN");
        defines.insert("PJ_BLOCKING_CONNECT_ERROR_VAL", "EINPROGRESS");
        defines.insert("PJ_HAS_HIGH_RES_TIMER", "1");
        defines.insert("PJ_HAS_MALLOC", "1");
        defines.insert("PJ_NATIVE_STRING_IS_UNICODE", "0");
        defines.insert("PJ_POOL_ALIGNMENT", "8");
        defines.insert("PJ_ATOMIC_VALUE_TYPE", "long");
        defines.insert("PJ_EMULATE_RWMUTEX", "0");
        defines.insert("PJ_THREAD_SET_STACK_SIZE", "0");
        defines.insert("PJ_THREAD_ALLOCATE_STACK", "0");
        defines.insert("PJ_TIMER_DEBUG", "0");

        // TODO : replace this hack with something else
        let define_from = "#define PJ_HAS_SYS_TIMEB_H	    1";
        let define_to = "#define PJ_HAS_SYS_TIMEB_H	    0";

        let os_linux = Path::new("./vendor/pjlib/include/pj/compat/os_linux.h");
        let mut src = File::open(os_linux).unwrap();

        let mut header_data = String::new();
        src.read_to_string(&mut header_data).unwrap();
        drop(src); // Close the file early

        // Run the replace operation in memory
        let new_header_data = header_data.replace(define_from, define_to);

        // Recreate the file and dump the processed contents to it
        let mut dst = File::create(os_linux).unwrap();
        dst.write_all(new_header_data.as_bytes()).unwrap();

        clang_flags.push(String::from("-DARM"));
        clang_flags.push(String::from("-DPJ_M_ARM64"));
        clang_flags.push(String::from("-DPJ_HAS_PENTIUM=0"));
        clang_flags.push(String::from("-DPJ_IS_BIG_ENDIAN=0"));
        clang_flags.push(String::from("-DPJ_IS_LITTLE_ENDIAN=1"));
        clang_flags.push(String::from("-DPJ_TIMER_DEBUG=0"));
    } else if target == "aarch64-unknown-linux-gnu" {
        defines.insert("ARM", "1");
        defines.insert("PJ_HAS_PENTIUM", "0");
        defines.insert("PJ_IS_BIG_ENDIAN", "0");
        defines.insert("PJ_IS_LITTLE_ENDIAN", "1");
        defines.insert("PJ_M_ARM64", "1");
        defines.insert("PJ_TIMER_DEBUG", "0");

        clang_flags.push(String::from("-DARM"));
        clang_flags.push(String::from("-DPJ_M_ARM64"));
        clang_flags.push(String::from("-DPJ_HAS_PENTIUM=0"));
        clang_flags.push(String::from("-DPJ_IS_BIG_ENDIAN=0"));
        clang_flags.push(String::from("-DPJ_IS_LITTLE_ENDIAN=1"));
        clang_flags.push(String::from("-DPJ_TIMER_DEBUG=0"));
    } else if target == "wasm32-unknown-emscripten" {
        defines.insert("PJ_LINUX", "1");
        defines.insert("PJ_M_X86_64", "1");

        clang_flags.push(String::from("-fvisibility=default"));
        clang_flags.push(String::from("-DPJ_LINUX=1"));
        clang_flags.push(String::from("-DPJ_M_X86_64=1"));

        let host = env::var("HOST").unwrap();
        let emscripten_path: &str;

        if host == "x86_64-unknown-linux-gnu" {
            emscripten_path = "/usr/share/emscripten/";
        } else if host == "x86_64-apple-darwin" {
            emscripten_path = "/usr/local/homebrew/opt/emscripten/libexec/";
        } else if host == "aarch64-apple-darwin" {
            emscripten_path = "/opt/homebrew/opt/emscripten/libexec/";
        } else {
            emscripten_path = "/usr/share/emscripten/";
        }

        let incl1 = format!("-I{}system/include/", emscripten_path);
        let incl2 = format!("-I{}system/lib/libc/musl/include/", emscripten_path);
        let incl3 = format!("-I{}system/lib/libc/musl/arch/emscripten/", emscripten_path);
        let incl4 = format!("-I{}system/lib/libc/musl/arch/generic/", emscripten_path);

        clang_flags.push(incl1);
        clang_flags.push(incl2);
        clang_flags.push(incl3);
        clang_flags.push(incl4);
    }

    // create an empty config file, as we're defining everything above
    File::create("vendor/pjlib/include/pj/config_site.h").unwrap();

    let mut pj_cmd = cc::Build::new();

    pj_cmd
        .warnings(false)
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
        .file("vendor/pjlib/src/pj/sock_bsd.c")
        .file("vendor/pjlib/src/pj/sock_common.c")
        .file("vendor/pjlib/src/pj/sock_qos_bsd.c")
        .file("vendor/pjlib/src/pj/sock_qos_common.c")
        .file("vendor/pjlib/src/pj/sock_qos_darwin.c")
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

    pj_cmd.define("PJ_HAS_NETINET_TCP_H", "1");

    pj_cmd.compile("pj");

    let mut pj_util_cmd = cc::Build::new();

    pj_util_cmd
        .warnings(false)
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
        .file("vendor/pjlib-util/src/pjlib-util/xml.c")
        .file("additional/src/os.c");

    for (key, value) in &defines {
        pj_util_cmd.define(key, *value);
    }

    pj_util_cmd.compile("pj-util");

    let mut pj_nath_cmd = cc::Build::new();

    pj_nath_cmd
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
        .file("additional/src/stun_config.c");

    for (key, value) in &defines {
        pj_nath_cmd.define(key, *value);
    }

    pj_nath_cmd.compile("pjnath");

    let mut pj_media_cmd = cc::Build::new();

    pj_media_cmd
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
        .include(additional_includes)
        .define("PJMEDIA_HAS_G711_CODEC", "0")
        .define("PJMEDIA_HAS_G722_CODEC", "0")
        .define("PJMEDIA_HAS_G7221_CODEC", "0")
        .define("PJMEDIA_HAS_GSM_CODEC", "0")
        .define("PJMEDIA_HAS_ILBC_CODEC", "0")
        .define("PJMEDIA_HAS_ILBC_CODEC", "0")
        .define("PJMEDIA_HAS_L16_CODEC", "0")
        .define("PJMEDIA_HAS_OPENCORE_AMRNB_CODEC", "0")
        .define("PJMEDIA_HAS_OPENCORE_AMRWB_CODEC", "0")
        .define("PJMEDIA_HAS_OPUS_CODEC", "0")
        .define("PJMEDIA_HAS_SILK_CODEC", "0")
        .define("PJMEDIA_HAS_SPEEX_CODEC", "0")
        .define("PJMEDIA_HAS_SPEEX_AEC", "0")
        .define("PJ_FUNCTIONS_ARE_INLINED", "0")
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
        .file("additional/src/circbuf.c");

    for (key, value) in &defines {
        pj_media_cmd.define(key, *value);
    }

    pj_media_cmd.compile("pjmedia");

    // generate the bindings for pjproject headers
    let mut builder = bindgen::Builder::default();

    for value in &clang_flags {
        builder = builder.clang_arg(value);
    }

    let bindings = builder
        .clang_arg("-Ivendor/pjlib/include/")
        .clang_arg("-Ivendor/pjlib-util/include/")
        .clang_arg("-Ivendor/pjnath/include/")
        .clang_arg("-Ivendor/pjmedia/include/")
        .clang_arg("-Ivendor/pjsip/include/")
        .clang_arg("-Iadditional/include/")
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
