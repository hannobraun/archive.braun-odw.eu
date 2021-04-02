mod args;
mod build;
mod serve;

use std::path::Path;

use self::{
    args::Args,
    build::{build, build_continuously},
    serve::serve_sites,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let source_dir = "sites";
    let output_dir = "output";

    let mut transform = Transform { dev: args.dev };

    if args.dev {
        let build = build_continuously(source_dir, output_dir, &mut transform);
        let serve = serve_sites(output_dir);
        tokio::try_join!(build, serve)?;
    } else {
        build(source_dir, output_dir, &mut transform).await?;
    }

    Ok(())
}

struct Transform {
    dev: bool,
}

impl build::Transform for Transform {
    fn transform(
        &mut self,
        _source: &Path,
        document: &mut kuchiki::NodeRef,
    ) -> anyhow::Result<()> {
        if self.dev {
            for base in document.select("base").unwrap() {
                *base.attributes.borrow_mut().get_mut("href").unwrap() =
                    String::from("/hanno.braun-odw.eu/");
            }
        }

        Ok(())
    }
}
