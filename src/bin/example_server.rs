use std::os::unix::net::UnixDatagram;
use std::thread;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() <= 1 {
        eprintln!("usage: {} <control socket path>\n", args[0]);
        return Err("missing required arg".into());
    }

    let control_socket_path = &args[1];
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

    let mut counter = 0;
    let mut buf = [0u8; 255];
    loop {
        let bytes_received = match data_channel.recv(&mut buf[..]) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("data channel error: {}", e);
                break;
            }
        };
        if bytes_received >= buf.len() {
            panic!("buffer too small!");
        }
        counter += 1;
        if counter % 100 == 0 {
            eprint!(".");
        }
    }
}
