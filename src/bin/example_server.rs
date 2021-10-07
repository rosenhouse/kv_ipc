use std::io::ErrorKind;
use std::os::unix::net::UnixDatagram;
use std::thread;

use clap::{AppSettings, Clap};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

/// Performance testing tool
#[derive(Clap)]
#[clap(version = "0.1")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// unix socket to listen on
    #[clap(long, default_value = "/tmp/kv-ipc-control-channel")]
    control_channel: String,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let control_socket_path = opts.control_channel;
    match std::fs::remove_file(&control_socket_path) {
        Ok(()) => (),
        Err(ref e) if e.kind() == ErrorKind::NotFound => (),
        Err(e) => panic!("remove file error for {}: {}", control_socket_path, e),
    }
    let control_channel = UnixDatagram::bind(control_socket_path)?;

    // control loop, handling requests for data channels
    loop {
        let (_, client_address) = control_channel.recv_from(&mut [])?;
        let client_path = client_address.as_pathname().unwrap();
        eprintln!("got request from client {:?}", client_path);

        let data_channel_path = kv_ipc::new_socket_path();
        eprintln!("binding to new path for data socket: {}", data_channel_path);
        let data_channel = UnixDatagram::bind(&data_channel_path).unwrap();

        eprintln!("client {:?} -> {}", client_address, data_channel_path,);

        // tell the client to reconnect to the data channel
        let encoded_path = &rmp_serde::to_vec(&data_channel_path)?;
        control_channel.send_to(encoded_path, client_path)?;

        thread::spawn(move || handle_data_channel(data_channel, data_channel_path));
    }
}

fn handle_data_channel(data_channel: UnixDatagram, data_channel_path: String) {
    // wait for a an empty message, then delete the file path
    let (signal_bytes, client_address) = data_channel.recv_from(&mut []).unwrap();
    if signal_bytes != 0 {
        panic!("signal had data");
    }
    eprintln!(
        "client signaled, connecting data channel {} to client {:?}",
        data_channel_path, client_address
    );
    data_channel
        .connect(client_address.as_pathname().unwrap())
        .unwrap();
    eprintln!("removing data file from filesystem",);
    // client is connected, so we can remove the path from the filesystem
    std::fs::remove_file(data_channel_path).unwrap();

    eprintln!("sending empty message back to signal ready");
    data_channel.send(&mut []).unwrap();

    data_channel
        .set_read_timeout(Some(std::time::Duration::from_millis(100)))
        .unwrap();

    let mut counter = 0;
    let mut buf = [0u8; 2550];
    loop {
        let recv_result = data_channel.recv(&mut buf[..]);
        let bytes_received = match recv_result {
            Ok(n) => n,
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                // timeout error
                eprintln!("testing if socket is still alive...");
                data_channel.set_nonblocking(true).unwrap();
                let send_result = data_channel.send(&mut []);
                data_channel.set_nonblocking(false).unwrap();
                match send_result {
                    Err(e) if e.kind() == ErrorKind::ConnectionRefused => {
                        eprintln!("peer closed the socket");
                        break;
                    }
                    Err(e) => {
                        eprintln!("healthcheck socket send error: {}", e);
                        break;
                    }
                    Ok(_) => {
                        // yes still alive, try again
                        continue;
                    }
                };
            }
            Err(e) => {
                eprintln!("read error, will break: {}", e);
                break;
            }
        };
        if bytes_received >= buf.len() {
            panic!("buffer too small!");
        }
        counter += 1;
        if buf[0] != (counter as u8) {
            panic!("missed a message");
        }
        if counter % 10000 == 0 {
            eprint!(".");
        }
    }

    eprintln!("received {} messages from {:?}", counter, client_address);
}
