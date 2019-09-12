extern crate ssh2;

use std::net::TcpStream;
use ssh2::Session;
use std::env;
use std::io::prelude::*;

static USERNAME: &'static str = "docker";
static REMOTE_ADDRESS: &'static str = "192.168.99.100:22";


fn main() {
    let tcp = TcpStream::connect(REMOTE_ADDRESS).unwrap();
    let mut session = Session::new().unwrap();
    session.handshake(&tcp).unwrap();
    let private_key_file_path = dirs::home_dir()
        .unwrap()
        .join(".docker/machine/machines/default/id_rsa");

    session.userauth_pubkey_file(
        USERNAME,
        None,
        private_key_file_path.as_path(),
        None,
    ).unwrap();

    assert!(session.authenticated());

    let mut chan = session.channel_session().unwrap();
    chan.exec("ls").unwrap();
    let mut s = String::new();

    chan.read_to_string(&mut s).unwrap();
    println!("{}", s);
}
