type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

use std::os::unix::net::UnixDatagram;

fn main() -> Result<()> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() <= 1 {
        eprintln!("usage: {} <control socket path>\n", args[0]);
        return Err("missing required arg".into());
    }

    let control_socket_path = &args[1];

    let local_path = kv_ipc::new_socket_path();

    eprintln!("binding to local socket {}", local_path);
    let socket = UnixDatagram::bind(&local_path).unwrap();

    eprintln!("connecting to control socket {}", control_socket_path);
    socket.connect(control_socket_path).unwrap();

    eprintln!("sending an empty message, to ask for a data socket");
    socket.send(&[])?;

    eprintln!("waiting for a response specifying the data socket location");
    let mut buf = [0u8; 150];
    let num_received = socket.recv(&mut buf[..])?;
    if num_received >= buf.len() {
        panic!("path too long");
    }
    // once rust supports passing fd via unix sockets,
    // then this gets a lot simpler
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

    eprintln!("starting to send messages...");
    let num_messages = 100000;
    for i in 1..num_messages {
        socket.send(&buf)?;

        if i % 100 == 0 {
            eprint!(".");
        }
    }

    eprintln!("sent {} messages", num_messages);
    Ok(())
}
