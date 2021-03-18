use std::net::SocketAddr;

use warp::{Filter, Rejection, Reply};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let hello = warp::path::end().map(|| format!("Hello, world!"));

    let ipv4 = serve(hello, "127.0.0.1:8000".parse()?);
    let ipv6 = serve(hello, "[::1]:8000".parse()?);

    tokio::join!(ipv4, ipv6);

    Ok(())

    // TASK: Serve hanno.braun-odw.eu.
    // TASK: Redirect from braun-odw.eu to hanno.braun-odw.eu.
    // TASK: Redirect from www.braun-odw.eu to hanno.braun-odw.eu.
}

async fn serve<F>(filter: F, addr: SocketAddr)
where
    F: Filter<Error = Rejection> + Clone + Send + Sync + 'static,
    F::Extract: Reply,
{
    warp::serve(filter).run(addr).await
}
