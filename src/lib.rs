mod client;
mod server;

pub use crate::client::KVClient;
pub use crate::server::KVServer;

use std::{borrow::Cow, os::unix::prelude::PermissionsExt};

use serde::{Deserialize, Serialize};

type IOResult<T> = std::io::Result<T>;
type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

/// A record passed from client to server
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct SerializableRecord<'a> {
    pub table_id: u32,

    #[serde(with = "serde_bytes")]
    #[serde(borrow)]
    pub key: Cow<'a, [u8]>,

    #[serde(with = "serde_bytes")]
    #[serde(borrow)]
    pub value: Cow<'a, [u8]>,
}

pub fn new_socket_path() -> String {
    let mut path = tempfile::TempDir::new().unwrap().into_path();
    let mut perms = std::fs::metadata(&path).unwrap().permissions();
    perms.set_mode(0x1777);
    path.push("socket");
    path.as_path().to_str().unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    #[test]
    fn round_trip() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("socket");
        let kvserver = crate::server::KVServer::bind(&socket_path).unwrap();

        let kvclient = crate::client::KVClient::connect(&socket_path).unwrap();

        let send1 = crate::SerializableRecord {
            table_id: 42,
            key: super::Cow::Owned(vec![1, 2, 3, 4]),
            value: super::Cow::Owned(vec![10, 20, 30, 40]),
        };

        kvclient.insert(&send1).unwrap();

        let mut buf = [0u8; 255];
        let receive1 = kvserver.receive(&mut buf[..]).unwrap();

        assert_eq!(send1, receive1);
    }

    #[test]
    fn simple_decode() {
        let buf = [147, 42, 148, 1, 2, 3, 4, 148, 10, 20, 30, 40];

        let got = rmp_serde::from_read_ref(&buf).unwrap();

        let send1 = crate::SerializableRecord {
            table_id: 42,
            key: super::Cow::Owned(vec![1, 2, 3, 4]),
            value: super::Cow::Owned(vec![10, 20, 30, 40]),
        };
        assert_eq!(send1, got);
    }
}
