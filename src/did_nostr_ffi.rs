//! C++ FFI bindings for DID-NOSTR functionality

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use crate::did_nostr::*;

/// Create a DID from NOSTR public key (hex)
/// # Safety
/// Caller must ensure pubkey_hex is a valid null-terminated string
#[no_mangle]
pub unsafe extern "C" fn did_nostr_create(
    pubkey_hex: *const c_char,
) -> *mut c_char {
    if pubkey_hex.is_null() {
        let err = CString::new("null pointer").unwrap();
        return err.into_raw();
    }

    let hex_cstr = match CStr::from_ptr(pubkey_hex).to_str() {
        Ok(s) => s,
        Err(_) => {
            let err = CString::new("invalid UTF-8").unwrap();
            return err.into_raw();
        }
    };

    match NostrPublicKey::from_hex(hex_cstr) {
        Ok(pubkey) => {
            let did = DidNostr::from_pubkey(pubkey);
            let result = CString::new(did.to_string()).unwrap();
            result.into_raw()
        }
        Err(e) => {
            let err = CString::new(format!("error: {}", e)).unwrap();
            err.into_raw()
        }
    }
}

/// Parse DID from string
/// # Safety
/// Caller must ensure did_str is a valid null-terminated string
#[no_mangle]
pub unsafe extern "C" fn did_nostr_parse(
    did_str: *const c_char,
) -> *mut c_char {
    if did_str.is_null() {
        let err = CString::new("null pointer").unwrap();
        return err.into_raw();
    }

    let did_cstr = match CStr::from_ptr(did_str).to_str() {
        Ok(s) => s,
        Err(_) => {
            let err = CString::new("invalid UTF-8").unwrap();
            return err.into_raw();
        }
    };

    match DidNostr::from_str(did_cstr) {
        Ok(did) => {
            let result = CString::new(did.to_string()).unwrap();
            result.into_raw()
        }
        Err(e) => {
            let err = CString::new(format!("error: {}", e)).unwrap();
            err.into_raw()
        }
    }
}

/// Extract public key from DID
/// # Safety
/// Caller must ensure did_str is a valid null-terminated string
#[no_mangle]
pub unsafe extern "C" fn did_nostr_get_pubkey(
    did_str: *const c_char,
) -> *mut c_char {
    if did_str.is_null() {
        let err = CString::new("null pointer").unwrap();
        return err.into_raw();
    }

    let did_cstr = match CStr::from_ptr(did_str).to_str() {
        Ok(s) => s,
        Err(_) => {
            let err = CString::new("invalid UTF-8").unwrap();
            return err.into_raw();
        }
    };

    match DidNostr::from_str(did_cstr) {
        Ok(did) => {
            let result = CString::new(did.pubkey().as_hex()).unwrap();
            result.into_raw()
        }
        Err(e) => {
            let err = CString::new(format!("error: {}", e)).unwrap();
            err.into_raw()
        }
    }
}

/// Free a string allocated by the Rust library
/// # Safety
/// Caller must ensure ptr is a valid pointer returned by did_nostr_* functions
#[no_mangle]
pub unsafe extern "C" fn did_nostr_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        let _ = CString::from_raw(ptr);
    }
}
