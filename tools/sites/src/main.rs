// TASK: Package as Nix package and deploy to Reineke:
//       https://github.com/NixOS/nixpkgs/blob/master/doc/languages-frameworks/rust.section.md#compiling-rust-applications-with-cargo

use std::net::{IpAddr, SocketAddr};

use warp::{Filter, Rejection, Reply};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let hello = warp::path::end().map(|| format!("Hello, world!"));

    let ipv4 = "127.0.0.1".parse()?;
    let ipv6 = "::1".parse()?;

    let ipv4 = serve(hello, ipv4);
    let ipv6 = serve(hello, ipv6);

    tokio::join!(ipv4, ipv6);

    Ok(())

    // TASK: Serve hanno.braun-odw.eu.
    // TASK: Redirect from braun-odw.eu to hanno.braun-odw.eu.
    // TASK: Redirect from www.braun-odw.eu to hanno.braun-odw.eu.
}

async fn serve<F>(filter: F, addr: IpAddr)
where
    F: Filter<Error = Rejection> + Clone + Send + Sync + 'static,
    F::Extract: Reply,
{
    let addr = SocketAddr::new(addr, 8000);
    warp::serve(filter).run(addr).await
}
