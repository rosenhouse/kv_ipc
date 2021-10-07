type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

use std::{net::Shutdown, os::unix::net::UnixDatagram};

use rand::RngCore;

fn get_data_channel(control_socket_path: &str) -> Result<UnixDatagram> {
    let local_path = kv_ipc::new_socket_path();

    eprintln!("binding to local socket {}", local_path);
    let socket = UnixDatagram::bind(&local_path).unwrap();

    eprintln!("connecting to control socket {}", control_socket_path);
    socket.connect(control_socket_path).unwrap();

    eprintln!("sending an empty message, to ask for a data socket");
    socket.send(&[])?;

    // once rust supports passing fd via unix sockets, this gets way simpler
    eprintln!("waiting for a response specifying the data socket location");
    let mut buf = [0u8; 255];
    let num_received = socket.recv(&mut buf[..])?;
    if num_received >= buf.len() {
        panic!("path too long");
    }
    let data_channel_path: String = rmp_serde::from_read_ref(&buf)?;

    eprintln!("connecting to data channel at {}", data_channel_path);
    socket.connect(data_channel_path)?;

    // send an empty message to signal successful connection
    // the server will delete the data channel file from the filesystem in order to
    // prevent other processes connecting
    eprintln!("sending empty message on data socket, to show connected");
    socket.send(&[])?;

    eprintln!("waiting for empty response, to show server ready");
    socket.recv(&mut [])?;

    // we can safely delete our local socket path, to prevent other processes
    // besides the server from sending to us
    eprintln!("removing local socket path {}", local_path);
    std::fs::remove_file(local_path).unwrap();

    Ok(socket)
}

use clap::{AppSettings, Clap};

/// Performance testing tool
#[derive(Clap)]
#[clap(version = "0.1")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// unix socket to connect to
    #[clap(long, default_value = "/tmp/kv-ipc-control-channel")]
    control_channel: String,

    /// Number of messages to send
    #[clap(short, long, default_value = "1000")]
    number: u32,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let control_socket_path = &opts.control_channel;
    let socket = get_data_channel(control_socket_path)?;

    eprintln!("starting to send messages...");

    use rand::SeedableRng;
    let mut rng = rand_pcg::Pcg64::seed_from_u64(0);
    let mut buf = [0u8; 155];
    let num_messages = opts.number;
    use std::io::ErrorKind;

    for i in 1..num_messages + 1 {
        rng.fill_bytes(&mut buf);
        buf[0] = i as u8;

        let bytes_sent = socket.send(&buf).unwrap();
        if bytes_sent != buf.len() {
            panic!("didn't send enough bytes");
        }

        // consume any healthcheck messages
        socket.set_nonblocking(true)?;
        loop {
            match socket.recv(&mut []) {
                Ok(_) => continue,
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    socket.set_nonblocking(false).unwrap();
                    break;
                }
                Err(e) => return Err(Box::new(e)),
            }
        }
        socket.set_nonblocking(false)?;

        if i % 10000 == 0 {
            eprint!(".");
        }
    }
    socket.shutdown(Shutdown::Both)?;

    eprintln!("sent {} messages", num_messages);
    Ok(())
}
