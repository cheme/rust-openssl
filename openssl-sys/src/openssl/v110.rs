use libc::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_void, size_t};
use std::ptr;
use std::sync::{Once, ONCE_INIT};

pub enum BIGNUM {}
pub enum BIO {}
pub enum BIO_METHOD {}
pub enum CRYPTO_EX_DATA {}
pub enum DH {}
pub enum DSA {}
pub enum ECDSA_SIG {}
pub enum EVP_CIPHER {}
pub enum EVP_MD_CTX {}
pub enum EVP_PKEY {}
pub enum HMAC_CTX {}
pub enum OPENSSL_STACK {}
pub enum PKCS12 {}
pub enum RSA {}
pub enum SSL {}
pub enum SSL_CTX {}
pub enum SSL_SESSION {}
pub enum stack_st_ASN1_OBJECT {}
pub enum stack_st_GENERAL_NAME {}
pub enum stack_st_OPENSSL_STRING {}
pub enum stack_st_void {}
pub enum stack_st_X509 {}
pub enum stack_st_X509_NAME {}
pub enum stack_st_X509_ATTRIBUTE {}
pub enum stack_st_X509_EXTENSION {}
pub enum stack_st_SSL_CIPHER {}
pub enum OPENSSL_INIT_SETTINGS {}
pub enum X509 {}
pub enum X509_ALGOR {}
pub enum X509_VERIFY_PARAM {}
pub enum X509_REQ {}

pub const CMS_ASCIICRLF: c_uint = 0x80000;

pub const SSL_CTRL_SET_MIN_PROTO_VERSION: c_int = 123;
pub const SSL_CTRL_SET_MAX_PROTO_VERSION: c_int = 124;
#[cfg(ossl110g)]
pub const SSL_CTRL_GET_MIN_PROTO_VERSION: c_int = 130;
#[cfg(ossl110g)]
pub const SSL_CTRL_GET_MAX_PROTO_VERSION: c_int = 131;

pub const SSL_OP_MICROSOFT_SESS_ID_BUG: c_ulong = 0x00000000;
pub const SSL_OP_NETSCAPE_CHALLENGE_BUG: c_ulong = 0x00000000;
pub const SSL_OP_NETSCAPE_REUSE_CIPHER_CHANGE_BUG: c_ulong = 0x00000000;
pub const SSL_OP_MICROSOFT_BIG_SSLV3_BUFFER: c_ulong = 0x00000000;
pub const SSL_OP_SSLEAY_080_CLIENT_DH_BUG: c_ulong = 0x00000000;
pub const SSL_OP_TLS_D5_BUG: c_ulong = 0x00000000;
pub const SSL_OP_TLS_BLOCK_PADDING_BUG: c_ulong = 0x00000000;
pub const SSL_OP_SINGLE_ECDH_USE: c_ulong = 0x00000000;
pub const SSL_OP_SINGLE_DH_USE: c_ulong = 0x00000000;
pub const SSL_OP_NO_SSLv2: c_ulong = 0x00000000;

pub const OPENSSL_VERSION: c_int = 0;
pub const OPENSSL_CFLAGS: c_int = 1;
pub const OPENSSL_BUILT_ON: c_int = 2;
pub const OPENSSL_PLATFORM: c_int = 3;
pub const OPENSSL_DIR: c_int = 4;

pub const CRYPTO_EX_INDEX_SSL: c_int = 0;
pub const CRYPTO_EX_INDEX_SSL_CTX: c_int = 1;

pub const OPENSSL_INIT_LOAD_SSL_STRINGS: u64 = 0x00200000;

pub const X509_V_ERR_DANE_NO_MATCH: c_int = 65;
pub const X509_V_ERR_EE_KEY_TOO_SMALL: c_int = 66;
pub const X509_V_ERR_CA_KEY_TOO_SMALL: c_int = 67;
pub const X509_V_ERR_CA_MD_TOO_WEAK: c_int = 68;
pub const X509_V_ERR_INVALID_CALL: c_int = 69;
pub const X509_V_ERR_STORE_LOOKUP: c_int = 70;
pub const X509_V_ERR_NO_VALID_SCTS: c_int = 71;

pub const X509_CHECK_FLAG_NEVER_CHECK_SUBJECT: c_uint = 0x20;

pub fn init() {
    // explicitly initialize to work around https://github.com/openssl/openssl/issues/3505
    static INIT: Once = ONCE_INIT;

    INIT.call_once(|| unsafe {
        OPENSSL_init_ssl(OPENSSL_INIT_LOAD_SSL_STRINGS, ptr::null_mut());
    })
}

pub unsafe fn SSL_CTX_set_min_proto_version(ctx: *mut ::SSL_CTX, version: c_int) -> c_int {
    ::SSL_CTX_ctrl(
        ctx,
        SSL_CTRL_SET_MIN_PROTO_VERSION,
        version as c_long,
        ptr::null_mut(),
    ) as c_int
}

pub unsafe fn SSL_CTX_set_max_proto_version(ctx: *mut ::SSL_CTX, version: c_int) -> c_int {
    ::SSL_CTX_ctrl(
        ctx,
        SSL_CTRL_SET_MAX_PROTO_VERSION,
        version as c_long,
        ptr::null_mut(),
    ) as c_int
}

#[cfg(ossl110g)]
pub unsafe fn SSL_CTX_get_min_proto_version(ctx: *mut ::SSL_CTX) -> c_int {
    ::SSL_CTX_ctrl(ctx, SSL_CTRL_GET_MIN_PROTO_VERSION, 0, ptr::null_mut()) as c_int
}

#[cfg(ossl110g)]
pub unsafe fn SSL_CTX_get_max_proto_version(ctx: *mut ::SSL_CTX) -> c_int {
    ::SSL_CTX_ctrl(ctx, SSL_CTRL_GET_MAX_PROTO_VERSION, 0, ptr::null_mut()) as c_int
}

pub unsafe fn SSL_set_min_proto_version(s: *mut ::SSL, version: c_int) -> c_int {
    ::SSL_ctrl(
        s,
        SSL_CTRL_SET_MIN_PROTO_VERSION,
        version as c_long,
        ptr::null_mut(),
    ) as c_int
}

pub unsafe fn SSL_set_max_proto_version(s: *mut ::SSL, version: c_int) -> c_int {
    ::SSL_ctrl(
        s,
        SSL_CTRL_SET_MAX_PROTO_VERSION,
        version as c_long,
        ptr::null_mut(),
    ) as c_int
}

#[cfg(ossl110g)]
pub unsafe fn SSL_get_min_proto_version(s: *mut ::SSL) -> c_int {
    ::SSL_ctrl(s, SSL_CTRL_GET_MIN_PROTO_VERSION, 0, ptr::null_mut()) as c_int
}

#[cfg(ossl110g)]
pub unsafe fn SSL_get_max_proto_version(s: *mut ::SSL) -> c_int {
    ::SSL_ctrl(s, SSL_CTRL_GET_MAX_PROTO_VERSION, 0, ptr::null_mut()) as c_int
}

extern "C" {
    pub fn BIO_new(type_: *const BIO_METHOD) -> *mut BIO;
    pub fn BIO_s_file() -> *const BIO_METHOD;
    pub fn BIO_s_mem() -> *const BIO_METHOD;

    pub fn BN_get_rfc2409_prime_768(bn: *mut BIGNUM) -> *mut BIGNUM;
    pub fn BN_get_rfc2409_prime_1024(bn: *mut BIGNUM) -> *mut BIGNUM;
    pub fn BN_get_rfc3526_prime_1536(bn: *mut BIGNUM) -> *mut BIGNUM;
    pub fn BN_get_rfc3526_prime_2048(bn: *mut BIGNUM) -> *mut BIGNUM;
    pub fn BN_get_rfc3526_prime_3072(bn: *mut BIGNUM) -> *mut BIGNUM;
    pub fn BN_get_rfc3526_prime_4096(bn: *mut BIGNUM) -> *mut BIGNUM;
    pub fn BN_get_rfc3526_prime_6144(bn: *mut BIGNUM) -> *mut BIGNUM;
    pub fn BN_get_rfc3526_prime_8192(bn: *mut BIGNUM) -> *mut BIGNUM;

    pub fn CRYPTO_malloc(num: size_t, file: *const c_char, line: c_int) -> *mut c_void;
    pub fn CRYPTO_free(buf: *mut c_void, file: *const c_char, line: c_int);

    pub fn EVP_chacha20() -> *const ::EVP_CIPHER;
    pub fn EVP_chacha20_poly1305() -> *const ::EVP_CIPHER;

    pub fn HMAC_CTX_new() -> *mut HMAC_CTX;
    pub fn HMAC_CTX_free(ctx: *mut HMAC_CTX);

    pub fn OCSP_cert_to_id(
        dgst: *const ::EVP_MD,
        subject: *const ::X509,
        issuer: *const ::X509,
    ) -> *mut ::OCSP_CERTID;

    pub fn TLS_method() -> *const ::SSL_METHOD;
    pub fn DTLS_method() -> *const ::SSL_METHOD;
    pub fn SSL_CIPHER_get_version(cipher: *const ::SSL_CIPHER) -> *const c_char;
    pub fn X509_get_subject_name(x: *const ::X509) -> *mut ::X509_NAME;
    pub fn X509_get_issuer_name(x: *const ::X509) -> *mut ::X509_NAME;
    pub fn X509_set1_notAfter(x: *mut ::X509, tm: *const ::ASN1_TIME) -> c_int;
    pub fn X509_set1_notBefore(x: *mut ::X509, tm: *const ::ASN1_TIME) -> c_int;
    pub fn X509_get_ext_d2i(
        x: *const ::X509,
        nid: c_int,
        crit: *mut c_int,
        idx: *mut c_int,
    ) -> *mut c_void;
    pub fn X509_NAME_add_entry_by_NID(
        x: *mut ::X509_NAME,
        field: c_int,
        ty: c_int,
        bytes: *const c_uchar,
        len: c_int,
        loc: c_int,
        set: c_int,
    ) -> c_int;
    pub fn X509_get_signature_nid(x: *const X509) -> c_int;
    pub fn X509_ALGOR_get0(
        paobj: *mut *const ::ASN1_OBJECT,
        pptype: *mut c_int,
        ppval: *mut *const c_void,
        alg: *const ::X509_ALGOR,
    );
    pub fn X509_NAME_entry_count(n: *const ::X509_NAME) -> c_int;
    pub fn X509_NAME_get_entry(n: *const ::X509_NAME, loc: c_int) -> *mut ::X509_NAME_ENTRY;
    pub fn X509_NAME_ENTRY_get_data(ne: *const ::X509_NAME_ENTRY) -> *mut ::ASN1_STRING;
    pub fn X509_NAME_ENTRY_get_object(ne: *const ::X509_NAME_ENTRY) -> *mut ::ASN1_OBJECT;
    pub fn X509V3_EXT_nconf_nid(
        conf: *mut ::CONF,
        ctx: *mut ::X509V3_CTX,
        ext_nid: c_int,
        value: *const c_char,
    ) -> *mut ::X509_EXTENSION;
    pub fn X509V3_EXT_nconf(
        conf: *mut ::CONF,
        ctx: *mut ::X509V3_CTX,
        name: *const c_char,
        value: *const c_char,
    ) -> *mut ::X509_EXTENSION;
    pub fn ASN1_STRING_to_UTF8(out: *mut *mut c_uchar, s: *const ::ASN1_STRING) -> c_int;
    pub fn BN_is_negative(b: *const ::BIGNUM) -> c_int;
    pub fn EVP_CIPHER_key_length(cipher: *const EVP_CIPHER) -> c_int;
    pub fn EVP_CIPHER_block_size(cipher: *const EVP_CIPHER) -> c_int;
    pub fn EVP_CIPHER_iv_length(cipher: *const EVP_CIPHER) -> c_int;
    pub fn EVP_PBE_scrypt(
        pass: *const c_char,
        passlen: size_t,
        salt: *const c_uchar,
        saltlen: size_t,
        N: u64,
        r: u64,
        p: u64,
        maxmem: u64,
        key: *mut c_uchar,
        keylen: size_t,
    ) -> c_int;
    pub fn DSA_get0_pqg(
        d: *const ::DSA,
        p: *mut *const ::BIGNUM,
        q: *mut *const ::BIGNUM,
        q: *mut *const ::BIGNUM,
    );
    pub fn DSA_set0_pqg(
        d: *mut ::DSA,
        p: *mut ::BIGNUM,
        q: *mut ::BIGNUM,
        q: *mut ::BIGNUM,
    ) -> c_int;
    pub fn DSA_get0_key(
        d: *const ::DSA,
        pub_key: *mut *const ::BIGNUM,
        priv_key: *mut *const ::BIGNUM,
    );
    pub fn DSA_set0_key(
        d: *mut ::DSA,
        pub_key: *mut ::BIGNUM,
        priv_key: *mut ::BIGNUM,
    ) -> c_int;
    pub fn RSA_get0_key(
        r: *const ::RSA,
        n: *mut *const ::BIGNUM,
        e: *mut *const ::BIGNUM,
        d: *mut *const ::BIGNUM,
    );
    pub fn RSA_get0_factors(r: *const ::RSA, p: *mut *const ::BIGNUM, q: *mut *const ::BIGNUM);
    pub fn RSA_get0_crt_params(
        r: *const ::RSA,
        dmp1: *mut *const ::BIGNUM,
        dmq1: *mut *const ::BIGNUM,
        iqmp: *mut *const ::BIGNUM,
    );
    pub fn RSA_set0_key(
        r: *mut ::RSA,
        n: *mut ::BIGNUM,
        e: *mut ::BIGNUM,
        d: *mut ::BIGNUM,
    ) -> c_int;
    pub fn RSA_set0_factors(r: *mut ::RSA, p: *mut ::BIGNUM, q: *mut ::BIGNUM) -> c_int;
    pub fn RSA_set0_crt_params(
        r: *mut ::RSA,
        dmp1: *mut ::BIGNUM,
        dmq1: *mut ::BIGNUM,
        iqmp: *mut ::BIGNUM,
    ) -> c_int;
    pub fn ASN1_STRING_get0_data(x: *const ::ASN1_STRING) -> *const c_uchar;
    pub fn OPENSSL_sk_num(stack: *const ::OPENSSL_STACK) -> c_int;
    pub fn OPENSSL_sk_value(stack: *const ::OPENSSL_STACK, idx: c_int) -> *mut c_void;
    pub fn SSL_CTX_get_options(ctx: *const ::SSL_CTX) -> c_ulong;
    pub fn SSL_CTX_set_options(ctx: *mut ::SSL_CTX, op: c_ulong) -> c_ulong;
    pub fn SSL_CTX_clear_options(ctx: *mut ::SSL_CTX, op: c_ulong) -> c_ulong;
    pub fn SSL_CTX_sess_set_get_cb(
        ctx: *mut ::SSL_CTX,
        get_session_cb: Option<
            unsafe extern "C" fn(*mut ::SSL, *const c_uchar, c_int, *mut c_int) -> *mut SSL_SESSION,
        >,
    );
    pub fn SSL_get_client_random(ssl: *const SSL, out: *mut c_uchar, len: size_t) -> size_t;
    pub fn SSL_get_server_random(ssl: *const SSL, out: *mut c_uchar, len: size_t) -> size_t;
    pub fn SSL_get0_verified_chain(ssl: *const SSL) -> *mut stack_st_X509;
    pub fn X509_getm_notAfter(x: *const ::X509) -> *mut ::ASN1_TIME;
    pub fn X509_getm_notBefore(x: *const ::X509) -> *mut ::ASN1_TIME;
    pub fn X509_get0_signature(
        psig: *mut *const ::ASN1_BIT_STRING,
        palg: *mut *const ::X509_ALGOR,
        x: *const ::X509,
    );
    pub fn DH_set0_pqg(
        dh: *mut ::DH,
        p: *mut ::BIGNUM,
        q: *mut ::BIGNUM,
        g: *mut ::BIGNUM,
    ) -> c_int;
    pub fn BIO_set_init(a: *mut ::BIO, init: c_int);
    pub fn BIO_set_data(a: *mut ::BIO, data: *mut c_void);
    pub fn BIO_get_data(a: *mut ::BIO) -> *mut c_void;
    pub fn BIO_meth_new(type_: c_int, name: *const c_char) -> *mut ::BIO_METHOD;
    pub fn BIO_meth_free(biom: *mut ::BIO_METHOD);
    // FIXME should wrap in Option
    pub fn BIO_meth_set_write(
        biom: *mut ::BIO_METHOD,
        write: unsafe extern "C" fn(*mut ::BIO, *const c_char, c_int) -> c_int,
    ) -> c_int;
    pub fn BIO_meth_set_read(
        biom: *mut ::BIO_METHOD,
        read: unsafe extern "C" fn(*mut ::BIO, *mut c_char, c_int) -> c_int,
    ) -> c_int;
    pub fn BIO_meth_set_puts(
        biom: *mut ::BIO_METHOD,
        read: unsafe extern "C" fn(*mut ::BIO, *const c_char) -> c_int,
    ) -> c_int;
    pub fn BIO_meth_set_ctrl(
        biom: *mut ::BIO_METHOD,
        read: unsafe extern "C" fn(*mut ::BIO, c_int, c_long, *mut c_void) -> c_long,
    ) -> c_int;
    pub fn BIO_meth_set_create(
        biom: *mut ::BIO_METHOD,
        create: unsafe extern "C" fn(*mut ::BIO) -> c_int,
    ) -> c_int;
    pub fn BIO_meth_set_destroy(
        biom: *mut ::BIO_METHOD,
        destroy: unsafe extern "C" fn(*mut ::BIO) -> c_int,
    ) -> c_int;
    pub fn CRYPTO_get_ex_new_index(
        class_index: c_int,
        argl: c_long,
        argp: *mut c_void,
        new_func: Option<::CRYPTO_EX_new>,
        dup_func: Option<::CRYPTO_EX_dup>,
        free_func: Option<::CRYPTO_EX_free>,
    ) -> c_int;
    pub fn X509_up_ref(x: *mut X509) -> c_int;
    pub fn SSL_CTX_up_ref(x: *mut SSL_CTX) -> c_int;
    pub fn SSL_session_reused(ssl: *mut SSL) -> c_int;
    pub fn SSL_SESSION_get_master_key(
        session: *const SSL_SESSION,
        out: *mut c_uchar,
        outlen: size_t,
    ) -> size_t;
    pub fn SSL_SESSION_up_ref(ses: *mut SSL_SESSION) -> c_int;
    pub fn X509_get0_extensions(req: *const ::X509) -> *const stack_st_X509_EXTENSION;
    pub fn X509_STORE_CTX_get0_chain(ctx: *mut ::X509_STORE_CTX) -> *mut stack_st_X509;
    pub fn EVP_MD_CTX_new() -> *mut EVP_MD_CTX;
    pub fn EVP_MD_CTX_free(ctx: *mut EVP_MD_CTX);
    pub fn EVP_PKEY_bits(key: *const EVP_PKEY) -> c_int;

    pub fn OpenSSL_version_num() -> c_ulong;
    pub fn OpenSSL_version(key: c_int) -> *const c_char;
    pub fn OPENSSL_init_ssl(opts: u64, settings: *const OPENSSL_INIT_SETTINGS) -> c_int;
    pub fn OPENSSL_sk_new_null() -> *mut ::OPENSSL_STACK;
    pub fn OPENSSL_sk_free(st: *mut ::OPENSSL_STACK);
    pub fn OPENSSL_sk_pop_free(
        st: *mut ::OPENSSL_STACK,
        free: Option<unsafe extern "C" fn(*mut c_void)>,
    );
    pub fn OPENSSL_sk_push(st: *mut ::OPENSSL_STACK, data: *const c_void) -> c_int;
    pub fn OPENSSL_sk_pop(st: *mut ::OPENSSL_STACK) -> *mut c_void;

    pub fn PKCS12_create(
        pass: *const c_char,
        friendly_name: *const c_char,
        pkey: *mut EVP_PKEY,
        cert: *mut X509,
        ca: *mut stack_st_X509,
        nid_key: c_int,
        nid_cert: c_int,
        iter: c_int,
        mac_iter: c_int,
        keytype: c_int,
    ) -> *mut PKCS12;
    pub fn X509_REQ_get_version(req: *const X509_REQ) -> c_long;
    pub fn X509_REQ_get_subject_name(req: *const X509_REQ) -> *mut ::X509_NAME;
    pub fn SSL_extension_supported(ext_type: c_uint) -> c_int;
    pub fn ECDSA_SIG_get0(sig: *const ECDSA_SIG, pr: *mut *const BIGNUM, ps: *mut *const BIGNUM);
    pub fn ECDSA_SIG_set0(sig: *mut ECDSA_SIG, pr: *mut BIGNUM, ps: *mut BIGNUM) -> c_int;

    pub fn SSL_CIPHER_get_cipher_nid(c: *const ::SSL_CIPHER) -> c_int;
    pub fn SSL_CIPHER_get_digest_nid(c: *const ::SSL_CIPHER) -> c_int;
}
