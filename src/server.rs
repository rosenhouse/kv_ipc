use std::os::unix::net::UnixDatagram;
use std::path::Path;

use crate::SerializableRecord;
use crate::{IOResult, Result};

pub struct KVServer {
    ud: UnixDatagram,
}

impl KVServer {
    pub fn bind<P: AsRef<Path>>(path: P) -> IOResult<Self> {
        let ud = UnixDatagram::bind(path)?;
        Ok(KVServer { ud })
    }

    pub fn receive<'a>(&self, buffer: &'a mut [u8]) -> Result<SerializableRecord<'a>> {
        let num_bytes_read = self.ud.recv(buffer)?;
        if num_bytes_read >= buffer.len() {
            panic!("receive truncated a message.  Grow your buffer");
        }
        let rec: SerializableRecord = rmp_serde::from_read_ref(buffer)?;
        Ok(rec)
    }
}
