use ::std::os::raw::c_char;
use std::ffi::CStr;

/// Connection to the Key/Value server
///
/// Fields are published so the caller can allocate memory of the correct size
/// but the caller should not use the fields themselves
#[derive(Copy, Clone)]
#[repr(C)]
pub struct KVConn {
    pub fd: i32,
}

/// Create a connection to the named Key/Value server
/// Keyed off a file descriptor
/// So it is safe for use by concurrent threads and for
/// use by child processes
///
/// Returns 0 on success or an OS errno
#[no_mangle]
pub extern "C" fn open_kvconn(ipc_server_name: *const c_char, conn_out: &mut KVConn) -> i32 {
    let ipc_name = unsafe { CStr::from_ptr(ipc_server_name) }.to_str().unwrap();
    eprintln!("ipc_client: connect({})", ipc_name);
    conn_out.fd = -1;
    // get the Unix Domain fd here, stash it on the KVConn
    42
}
