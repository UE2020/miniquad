/* automatically generated by rust-bindgen 0.55.1 */

pub const __GBM__: u32 = 1;
pub const _STDINT_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 28;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __WORDSIZE: u32 = 32;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 0;
pub const __WORDSIZE32_SIZE_ULONG: u32 = 0;
pub const __WORDSIZE32_PTRDIFF_LONG: u32 = 0;
pub const __NO_LONG_DOUBLE_MATH: u32 = 1;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
pub const _BITS_TYPES_H: u32 = 1;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 0;
pub const __FD_SETSIZE: u32 = 1024;
pub const _BITS_WCHAR_H: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i32 = -2147483648;
pub const INT_FAST32_MIN: i32 = -2147483648;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u32 = 2147483647;
pub const INT_FAST32_MAX: u32 = 2147483647;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: u32 = 4294967295;
pub const UINT_FAST32_MAX: u32 = 4294967295;
pub const INTPTR_MIN: i32 = -2147483648;
pub const INTPTR_MAX: u32 = 2147483647;
pub const UINTPTR_MAX: u32 = 4294967295;
pub const PTRDIFF_MIN: i32 = -2147483648;
pub const PTRDIFF_MAX: u32 = 2147483647;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: u32 = 4294967295;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const GBM_FORMAT_BIG_ENDIAN: u32 = 2147483648;
pub const GBM_BO_IMPORT_WL_BUFFER: u32 = 21761;
pub const GBM_BO_IMPORT_EGL_IMAGE: u32 = 21762;
pub const GBM_BO_IMPORT_FD: u32 = 21763;
pub const GBM_BO_IMPORT_FD_MODIFIER: u32 = 21764;
pub const GBM_MAX_PLANES: u32 = 4;
pub type size_t = ::std::os::raw::c_uint;
pub type wchar_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __clang_max_align_nonce2: f64,
}
#[test]
fn bindgen_test_layout_max_align_t() {
    assert_eq!(
        ::std::mem::size_of::<max_align_t>(),
        16usize,
        concat!("Size of: ", stringify!(max_align_t))
    );
    assert_eq!(
        ::std::mem::align_of::<max_align_t>(),
        8usize,
        concat!("Alignment of ", stringify!(max_align_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<max_align_t>())).__clang_max_align_nonce1 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<max_align_t>())).__clang_max_align_nonce2 as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce2)
        )
    );
}
pub type __u_char = ::std::os::raw::c_uchar;
pub type __u_short = ::std::os::raw::c_ushort;
pub type __u_int = ::std::os::raw::c_uint;
pub type __u_long = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_longlong;
pub type __uint64_t = ::std::os::raw::c_ulonglong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = ::std::os::raw::c_longlong;
pub type __u_quad_t = ::std::os::raw::c_ulonglong;
pub type __intmax_t = ::std::os::raw::c_longlong;
pub type __uintmax_t = ::std::os::raw::c_ulonglong;
pub type __dev_t = __u_quad_t;
pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __ino_t = ::std::os::raw::c_ulong;
pub type __ino64_t = __u_quad_t;
pub type __mode_t = ::std::os::raw::c_uint;
pub type __nlink_t = ::std::os::raw::c_uint;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = __quad_t;
pub type __pid_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___fsid_t() {
    assert_eq!(
        ::std::mem::size_of::<__fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__fsid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__fsid_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__fsid_t>())).__val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__fsid_t),
            "::",
            stringify!(__val)
        )
    );
}
pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = ::std::os::raw::c_ulong;
pub type __rlim64_t = __u_quad_t;
pub type __id_t = ::std::os::raw::c_uint;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = __quad_t;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = __u_quad_t;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = __u_quad_t;
pub type __fsword_t = ::std::os::raw::c_int;
pub type __ssize_t = ::std::os::raw::c_int;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_int;
pub type __socklen_t = ::std::os::raw::c_uint;
pub type __sig_atomic_t = ::std::os::raw::c_int;
pub type int_least8_t = __int_least8_t;
pub type int_least16_t = __int_least16_t;
pub type int_least32_t = __int_least32_t;
pub type int_least64_t = __int_least64_t;
pub type uint_least8_t = __uint_least8_t;
pub type uint_least16_t = __uint_least16_t;
pub type uint_least32_t = __uint_least32_t;
pub type uint_least64_t = __uint_least64_t;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_int;
pub type int_fast32_t = ::std::os::raw::c_int;
pub type int_fast64_t = ::std::os::raw::c_longlong;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_uint;
pub type uint_fast32_t = ::std::os::raw::c_uint;
pub type uint_fast64_t = ::std::os::raw::c_ulonglong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
#[doc = " \\file gbm.h"]
#[doc = " \\brief Generic Buffer Manager"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gbm_device {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gbm_bo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gbm_surface {
    _unused: [u8; 0],
}
#[doc = " Abstraction representing the handle to a buffer allocated by the"]
#[doc = " manager"]
#[repr(C)]
#[derive(Copy, Clone)]
pub union gbm_bo_handle {
    pub ptr: *mut ::std::os::raw::c_void,
    pub s32: i32,
    pub u32_: u32,
    pub s64: i64,
    pub u64_: u64,
    //_bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_gbm_bo_handle() {
    assert_eq!(
        ::std::mem::size_of::<gbm_bo_handle>(),
        8usize,
        concat!("Size of: ", stringify!(gbm_bo_handle))
    );
    assert_eq!(
        ::std::mem::align_of::<gbm_bo_handle>(),
        8usize,
        concat!("Alignment of ", stringify!(gbm_bo_handle))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gbm_bo_handle>())).ptr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_bo_handle),
            "::",
            stringify!(ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gbm_bo_handle>())).s32 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_bo_handle),
            "::",
            stringify!(s32)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gbm_bo_handle>())).u32_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_bo_handle),
            "::",
            stringify!(u32_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gbm_bo_handle>())).s64 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_bo_handle),
            "::",
            stringify!(s64)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gbm_bo_handle>())).u64_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_bo_handle),
            "::",
            stringify!(u64_)
        )
    );
}
#[doc = " RGB with 8 bits per channel in a 32 bit value"]
pub const gbm_bo_format_GBM_BO_FORMAT_XRGB8888: gbm_bo_format = 0;
#[doc = " ARGB with 8 bits per channel in a 32 bit value"]
pub const gbm_bo_format_GBM_BO_FORMAT_ARGB8888: gbm_bo_format = 1;
#[doc = " Format of the allocated buffer"]
pub type gbm_bo_format = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gbm_format_name_desc {
    pub name: [::std::os::raw::c_char; 5usize],
}
#[test]
fn bindgen_test_layout_gbm_format_name_desc() {
    assert_eq!(
        ::std::mem::size_of::<gbm_format_name_desc>(),
        5usize,
        concat!("Size of: ", stringify!(gbm_format_name_desc))
    );
    assert_eq!(
        ::std::mem::align_of::<gbm_format_name_desc>(),
        1usize,
        concat!("Alignment of ", stringify!(gbm_format_name_desc))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gbm_format_name_desc>())).name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_format_name_desc),
            "::",
            stringify!(name)
        )
    );
}
#[doc = " Buffer is going to be presented to the screen using an API such as KMS"]
pub const gbm_bo_flags_GBM_BO_USE_SCANOUT: gbm_bo_flags = 1;
#[doc = " Buffer is going to be used as cursor"]
pub const gbm_bo_flags_GBM_BO_USE_CURSOR: gbm_bo_flags = 2;
#[doc = " Deprecated"]
pub const gbm_bo_flags_GBM_BO_USE_CURSOR_64X64: gbm_bo_flags = 2;
#[doc = " Buffer is to be used for rendering - for example it is going to be used"]
#[doc = " as the storage for a color buffer"]
pub const gbm_bo_flags_GBM_BO_USE_RENDERING: gbm_bo_flags = 4;
#[doc = " Buffer can be used for gbm_bo_write.  This is guaranteed to work"]
#[doc = " with GBM_BO_USE_CURSOR, but may not work for other combinations."]
pub const gbm_bo_flags_GBM_BO_USE_WRITE: gbm_bo_flags = 8;
#[doc = " Buffer is linear, i.e. not tiled."]
pub const gbm_bo_flags_GBM_BO_USE_LINEAR: gbm_bo_flags = 16;
#[doc = " Flags to indicate the intended use for the buffer - these are passed into"]
#[doc = " gbm_bo_create(). The caller must set the union of all the flags that are"]
#[doc = " appropriate"]
#[doc = ""]
#[doc = " \\sa Use gbm_device_is_format_supported() to check if the combination of format"]
#[doc = " and use flags are supported"]
pub type gbm_bo_flags = ::std::os::raw::c_uint;
extern "C" {
    pub fn gbm_device_get_fd(gbm: *mut gbm_device) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gbm_device_get_backend_name(gbm: *mut gbm_device) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn gbm_device_is_format_supported(
        gbm: *mut gbm_device,
        format: u32,
        usage: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gbm_device_get_format_modifier_plane_count(
        gbm: *mut gbm_device,
        format: u32,
        modifier: u64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gbm_device_destroy(gbm: *mut gbm_device);
}
extern "C" {
    pub fn gbm_create_device(fd: ::std::os::raw::c_int) -> *mut gbm_device;
}
extern "C" {
    pub fn gbm_bo_create(
        gbm: *mut gbm_device,
        width: u32,
        height: u32,
        format: u32,
        flags: u32,
    ) -> *mut gbm_bo;
}
extern "C" {
    pub fn gbm_bo_create_with_modifiers(
        gbm: *mut gbm_device,
        width: u32,
        height: u32,
        format: u32,
        modifiers: *const u64,
        count: ::std::os::raw::c_uint,
    ) -> *mut gbm_bo;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gbm_import_fd_data {
    pub fd: ::std::os::raw::c_int,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
}
#[test]
fn bindgen_test_layout_gbm_import_fd_data() {
    assert_eq!(
        ::std::mem::size_of::<gbm_import_fd_data>(),
        20usize,
        concat!("Size of: ", stringify!(gbm_import_fd_data))
    );
    assert_eq!(
        ::std::mem::align_of::<gbm_import_fd_data>(),
        4usize,
        concat!("Alignment of ", stringify!(gbm_import_fd_data))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gbm_import_fd_data>())).fd as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_data),
            "::",
            stringify!(fd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gbm_import_fd_data>())).width as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_data),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gbm_import_fd_data>())).height as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_data),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gbm_import_fd_data>())).stride as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_data),
            "::",
            stringify!(stride)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gbm_import_fd_data>())).format as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_data),
            "::",
            stringify!(format)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gbm_import_fd_modifier_data {
    pub width: u32,
    pub height: u32,
    pub format: u32,
    pub num_fds: u32,
    pub fds: [::std::os::raw::c_int; 4usize],
    pub strides: [::std::os::raw::c_int; 4usize],
    pub offsets: [::std::os::raw::c_int; 4usize],
    pub modifier: u64,
}
#[test]
fn bindgen_test_layout_gbm_import_fd_modifier_data() {
    assert_eq!(
        ::std::mem::size_of::<gbm_import_fd_modifier_data>(),
        72usize,
        concat!("Size of: ", stringify!(gbm_import_fd_modifier_data))
    );
    assert_eq!(
        ::std::mem::align_of::<gbm_import_fd_modifier_data>(),
        8usize,
        concat!("Alignment of ", stringify!(gbm_import_fd_modifier_data))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<gbm_import_fd_modifier_data>())).width as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_modifier_data),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<gbm_import_fd_modifier_data>())).height as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_modifier_data),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<gbm_import_fd_modifier_data>())).format as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_modifier_data),
            "::",
            stringify!(format)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<gbm_import_fd_modifier_data>())).num_fds as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_modifier_data),
            "::",
            stringify!(num_fds)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gbm_import_fd_modifier_data>())).fds as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_modifier_data),
            "::",
            stringify!(fds)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<gbm_import_fd_modifier_data>())).strides as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_modifier_data),
            "::",
            stringify!(strides)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<gbm_import_fd_modifier_data>())).offsets as *const _ as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_modifier_data),
            "::",
            stringify!(offsets)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<gbm_import_fd_modifier_data>())).modifier as *const _ as usize
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_modifier_data),
            "::",
            stringify!(modifier)
        )
    );
}
extern "C" {
    pub fn gbm_bo_import(
        gbm: *mut gbm_device,
        type_: u32,
        buffer: *mut ::std::os::raw::c_void,
        usage: u32,
    ) -> *mut gbm_bo;
}
#[doc = " Buffer contents read back (or accessed directly) at transfer"]
#[doc = " create time."]
pub const gbm_bo_transfer_flags_GBM_BO_TRANSFER_READ: gbm_bo_transfer_flags = 1;
#[doc = " Buffer contents will be written back at unmap time"]
#[doc = " (or modified as a result of being accessed directly)."]
pub const gbm_bo_transfer_flags_GBM_BO_TRANSFER_WRITE: gbm_bo_transfer_flags = 2;
#[doc = " Read/modify/write"]
pub const gbm_bo_transfer_flags_GBM_BO_TRANSFER_READ_WRITE: gbm_bo_transfer_flags = 3;
#[doc = " Flags to indicate the type of mapping for the buffer - these are"]
#[doc = " passed into gbm_bo_map(). The caller must set the union of all the"]
#[doc = " flags that are appropriate."]
#[doc = ""]
#[doc = " These flags are independent of the GBM_BO_USE_* creation flags. However,"]
#[doc = " mapping the buffer may require copying to/from a staging buffer."]
#[doc = ""]
#[doc = " See also: pipe_transfer_usage"]
pub type gbm_bo_transfer_flags = ::std::os::raw::c_uint;
extern "C" {
    pub fn gbm_bo_map(
        bo: *mut gbm_bo,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        flags: u32,
        stride: *mut u32,
        map_data: *mut *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn gbm_bo_unmap(bo: *mut gbm_bo, map_data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn gbm_bo_get_width(bo: *mut gbm_bo) -> u32;
}
extern "C" {
    pub fn gbm_bo_get_height(bo: *mut gbm_bo) -> u32;
}
extern "C" {
    pub fn gbm_bo_get_stride(bo: *mut gbm_bo) -> u32;
}
extern "C" {
    pub fn gbm_bo_get_stride_for_plane(bo: *mut gbm_bo, plane: ::std::os::raw::c_int) -> u32;
}
extern "C" {
    pub fn gbm_bo_get_format(bo: *mut gbm_bo) -> u32;
}
extern "C" {
    pub fn gbm_bo_get_bpp(bo: *mut gbm_bo) -> u32;
}
extern "C" {
    pub fn gbm_bo_get_offset(bo: *mut gbm_bo, plane: ::std::os::raw::c_int) -> u32;
}
extern "C" {
    pub fn gbm_bo_get_device(bo: *mut gbm_bo) -> *mut gbm_device;
}
extern "C" {
    pub fn gbm_bo_get_handle(bo: *mut gbm_bo) -> gbm_bo_handle;
}
extern "C" {
    pub fn gbm_bo_get_fd(bo: *mut gbm_bo) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gbm_bo_get_modifier(bo: *mut gbm_bo) -> u64;
}
extern "C" {
    pub fn gbm_bo_get_plane_count(bo: *mut gbm_bo) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gbm_bo_get_handle_for_plane(
        bo: *mut gbm_bo,
        plane: ::std::os::raw::c_int,
    ) -> gbm_bo_handle;
}
extern "C" {
    pub fn gbm_bo_write(
        bo: *mut gbm_bo,
        buf: *const ::std::os::raw::c_void,
        count: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gbm_bo_set_user_data(
        bo: *mut gbm_bo,
        data: *mut ::std::os::raw::c_void,
        destroy_user_data: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut gbm_bo, arg2: *mut ::std::os::raw::c_void),
        >,
    );
}
extern "C" {
    pub fn gbm_bo_get_user_data(bo: *mut gbm_bo) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn gbm_bo_destroy(bo: *mut gbm_bo);
}
extern "C" {
    pub fn gbm_surface_create(
        gbm: *mut gbm_device,
        width: u32,
        height: u32,
        format: u32,
        flags: u32,
    ) -> *mut gbm_surface;
}
extern "C" {
    pub fn gbm_surface_create_with_modifiers(
        gbm: *mut gbm_device,
        width: u32,
        height: u32,
        format: u32,
        modifiers: *const u64,
        count: ::std::os::raw::c_uint,
    ) -> *mut gbm_surface;
}
extern "C" {
    pub fn gbm_surface_lock_front_buffer(surface: *mut gbm_surface) -> *mut gbm_bo;
}
extern "C" {
    pub fn gbm_surface_release_buffer(surface: *mut gbm_surface, bo: *mut gbm_bo);
}
extern "C" {
    pub fn gbm_surface_has_free_buffers(surface: *mut gbm_surface) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gbm_surface_destroy(surface: *mut gbm_surface);
}
extern "C" {
    pub fn gbm_format_get_name(
        gbm_format: u32,
        desc: *mut gbm_format_name_desc,
    ) -> *mut ::std::os::raw::c_char;
}