use std::net::SocketAddr;

use warp::Filter as _;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let hello = warp::path::end().map(|| format!("Hello, world!"));

    let ipv4: SocketAddr = "127.0.0.1:8000".parse()?;
    let ipv4 = warp::serve(hello).run(ipv4);

    let ipv6: SocketAddr = "[::1]:8000".parse()?;
    let ipv6 = warp::serve(hello).run(ipv6);

    tokio::join!(ipv4, ipv6);

    Ok(())

    // TASK: Serve hanno.braun-odw.eu.
    // TASK: Redirect from braun-odw.eu to hanno.braun-odw.eu.
    // TASK: Redirect from www.braun-odw.eu to hanno.braun-odw.eu.
}
