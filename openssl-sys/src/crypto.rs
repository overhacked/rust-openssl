use libc::*;
use *;

#[cfg(ossl110)]
#[inline]
#[track_caller]
pub unsafe fn OPENSSL_malloc(num: usize) -> *mut c_void {
    CRYPTO_malloc(
        num,
        concat!(file!(), "\0").as_ptr() as *const _,
        line!() as _,
    )
}

#[cfg(not(ossl110))]
#[inline]
#[track_caller]
pub unsafe fn OPENSSL_malloc(num: c_int) -> *mut c_void {
    CRYPTO_malloc(
        num,
        concat!(file!(), "\0").as_ptr() as *const _,
        line!() as _,
    )
}

#[cfg(ossl110)]
#[inline]
#[track_caller]
pub unsafe fn OPENSSL_free(addr: *mut c_void) {
    CRYPTO_free(
        addr,
        concat!(file!(), "\0").as_ptr() as *const _,
        line!() as _,
    )
}

#[cfg(not(ossl110))]
#[inline]
pub unsafe fn OPENSSL_free(addr: *mut c_void) {
    CRYPTO_free(addr)
}

#[cfg(not(ossl110))]
pub const CRYPTO_LOCK_X509: c_int = 3;
#[cfg(not(ossl110))]
pub const CRYPTO_LOCK_EVP_PKEY: c_int = 10;
#[cfg(not(ossl110))]
pub const CRYPTO_LOCK_SSL_CTX: c_int = 12;
#[cfg(not(ossl110))]
pub const CRYPTO_LOCK_SSL_SESSION: c_int = 14;

cfg_if! {
    if #[cfg(ossl110)] {
        pub const CRYPTO_EX_INDEX_SSL: c_int = 0;
        pub const CRYPTO_EX_INDEX_SSL_CTX: c_int = 1;
    } else if #[cfg(libressl)] {
        pub const CRYPTO_EX_INDEX_SSL: c_int = 1;
        pub const CRYPTO_EX_INDEX_SSL_CTX: c_int = 2;
    }
}

cfg_if! {
    if #[cfg(any(ossl110, libressl271))] {
        pub const OPENSSL_VERSION: c_int = 0;
        pub const OPENSSL_CFLAGS: c_int = 1;
        pub const OPENSSL_BUILT_ON: c_int = 2;
        pub const OPENSSL_PLATFORM: c_int = 3;
        pub const OPENSSL_DIR: c_int = 4;
    } else {
        pub const SSLEAY_VERSION: c_int = 0;
        pub const SSLEAY_CFLAGS: c_int = 2;
        pub const SSLEAY_BUILT_ON: c_int = 3;
        pub const SSLEAY_PLATFORM: c_int = 4;
        pub const SSLEAY_DIR: c_int = 5;
    }
}

pub const CRYPTO_LOCK: c_int = 1;
