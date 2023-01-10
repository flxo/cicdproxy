use std::{process::Stdio, env};

use anyhow::Result;
use log::{debug, info, warn};
use tokio::{
    io::{self, AsyncBufReadExt},
    net, select, task,
};

const BOUNDARY_PROXY_ADDR: &str = "127.0.0.1:44787";
const LISTEN_ADDR: &str = "10.37.129.2:44787";

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    info!("Starting cicd proxy");

    task::spawn(async move {
        info!("Starting cicd boundary client");
        let cicd_boundary_client =
            tokio::process::Command::new("cicd-boundary-client-0.7.6a_mac-amd64")
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("failed to spawn cicd-boundary-client");
        let mut stdout = io::BufReader::new(cicd_boundary_client.stdout.unwrap()).lines();
        let mut stderr = io::BufReader::new(cicd_boundary_client.stderr.unwrap()).lines();
        loop {
            select! {
                Ok(Some(line)) = stdout.next_line() => debug!("cicd client: {line}"),
                Ok(Some(line)) = stderr.next_line() => warn!("cicd client: {line}"),
                else => panic!("failed to read stdout/stderr from cicd client"),
            }
        }
    });

    info!("Binding {LISTEN_ADDR}");
    let listener = net::TcpListener::bind(LISTEN_ADDR).await?;
    loop {
        let (mut stream, addr) = listener.accept().await?;
        let forward = async move {
            info!("New connection from {}", addr);
            info!("Connecting to cicd boundary {BOUNDARY_PROXY_ADDR}");
            let mut boundary_client = net::TcpStream::connect(BOUNDARY_PROXY_ADDR)
                .await
                .expect("failed to connect to boundary proxy");
            info!("Forwarding {BOUNDARY_PROXY_ADDR} <-> {addr}");
            io::copy_bidirectional(&mut stream, &mut boundary_client)
                .await
                .expect("Failed to forward");
            info!("Finished {BOUNDARY_PROXY_ADDR} <-> {addr}");
        };
        task::spawn(forward);
    }
}
