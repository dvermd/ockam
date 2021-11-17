use std::io::stdin;

pub const BOB_TCP_ADDRESS: &str = "127.0.0.1:4222";
pub const BOB_LISTENER_ADDRESS: &str = "office_listener";
pub const GITHUB_SSH_CRED_VERIFIER: &str = "github_ssh_cred_verifier";
pub const ECHOER: &str = "echoer";

pub fn read_line() -> String {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.replace(&['\n', '\r'][..], "")
}
