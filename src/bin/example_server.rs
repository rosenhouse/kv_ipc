type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() <= 1 {
        eprintln!("usage: {} <socket path>\n", args[0]);
        return Err("missing required arg".into());
    }

    let socket_path = &args[1];
    let server = match kv_ipc::KVServer::bind(socket_path) {
        Ok(s) => Ok(s),
        Err(e) => {
            if let Some(raw_os_err) = e.raw_os_error() {
                if raw_os_err == 48 {
                    eprintln!("remove the file first");
                }
            }
            Err(e)
        }
    }?;

    eprintln!("listening for messages on {}...", socket_path);
    let mut buf = [0u8; 255];
    loop {
        let received = server.receive(&mut buf[..])?;
        eprintln!("{:?}", received);
    }
}
