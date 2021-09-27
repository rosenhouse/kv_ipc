use std::borrow::Cow;
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::os::unix::io::IntoRawFd;
use std::os::unix::net::UnixDatagram;
use std::path::Path;
use std::slice::from_raw_parts;
use std::{ffi::CStr, os::unix::prelude::FromRawFd};

use crate::SerializableRecord;
use crate::{IOResult, Result};

/// Record that may be inserted into the server
#[repr(C)]
pub struct KVClientRecord {
    pub table_id: u32,
    pub key_size: usize,
    pub key_data: *const u8,
    pub value_size: usize,
    pub value_data: *const u8,
}

/// Connection to the Key/Value server
///
/// Fields are published so the caller can allocate memory of the correct size
/// but the caller should not use the fields themselves
#[repr(C)]
pub struct KVClient {
    fd: c_int,
}

/// Create a connection to the named Key/Value server
///
/// Keyed off a file descriptor
/// So it is safe for use by concurrent threads and for use by child processes
///
/// Returns 0 on success
#[no_mangle]
pub extern "C" fn kvclient_open(path: *const c_char, kvclient_out: &mut KVClient) -> c_int {
    let path = unsafe { CStr::from_ptr(path) }.to_str().unwrap();
    match KVClient::connect(path) {
        Ok(kvclient) => {
            *kvclient_out = kvclient;
            return 0;
        }
        Err(err) => {
            eprintln!("libkv_ipc_client: kvclient_open: error: {}", err);
            return err.raw_os_error().unwrap_or(-1);
        }
    }
}

/// Insert a record into the server
///
/// Returns 0 on success
#[no_mangle]
pub extern "C" fn kvclient_insert(kvclient: KVClient, record: &KVClientRecord) -> c_int {
    let record = SerializableRecord {
        table_id: record.table_id,
        key: Cow::Borrowed(unsafe { from_raw_parts(record.key_data, record.key_size) }),
        value: Cow::Borrowed(unsafe { from_raw_parts(record.value_data, record.value_size) }),
    };
    match kvclient.insert(&record) {
        Ok(()) => {
            return 0;
        }
        Err(err) => {
            eprintln!("libkv_ipc_client: kvclient_insert: error: {}", err);
            return -1;
        }
    }
}

impl KVClient {
    pub fn connect<P: AsRef<Path>>(path: P) -> IOResult<Self> {
        let ud = UnixDatagram::unbound()?;
        ud.connect(path)?;
        Ok(KVClient {
            fd: ud.into_raw_fd(),
        })
    }

    pub fn insert(self, record: &SerializableRecord) -> Result<()> {
        let buf = rmp_serde::to_vec(record)?;

        // move fd into ud
        let ud = unsafe { UnixDatagram::from_raw_fd(self.fd) };

        // do the thing
        let res = ud.send(&buf);

        // move fd back out of ud, so it doesn't get dropped
        let _ = ud.into_raw_fd();

        res?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::prelude::AsRawFd;

    #[test]
    fn roundtrip_ud_via_fd() -> Result<()> {
        let ud = UnixDatagram::unbound()?;

        let orig_fd = ud.as_raw_fd();

        // move fd out of ud
        let new_owning_fd = ud.into_raw_fd();

        // resurrect ud
        let ud2 = unsafe { UnixDatagram::from_raw_fd(new_owning_fd) };

        // move fd back out of ud, so it doesn't get dropped
        let final_fd = ud2.into_raw_fd();

        assert_eq!(orig_fd, final_fd);

        Ok(())
    }
}
