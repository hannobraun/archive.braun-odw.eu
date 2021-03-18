use std::net::SocketAddr;

use warp::Filter as _;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let hello = warp::path::end().map(|| format!("Hello, world!"));

    // TASK: Also serve via IPv4.
    let ipv6: SocketAddr = "[::1]:8000".parse()?;
    warp::serve(hello).run(ipv6).await;

    Ok(())

    // TASK: Serve hanno.braun-odw.eu.
    // TASK: Redirect from braun-odw.eu to hanno.braun-odw.eu.
    // TASK: Redirect from www.braun-odw.eu to hanno.braun-odw.eu.
}
