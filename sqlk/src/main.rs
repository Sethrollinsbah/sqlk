use anyhow::Result;
use clap::Parser;
use sqlk::application::app::App;

#[tokio::main]
async fn main() -> Result<()> {
    let args = sqlk::args::Args::parse();

    if args.env.exists() {
        dotenv::from_path(&args.env).ok();
    }

    let mut app = App::new(args).await?;
    app.run().await?;

    Ok(())
}
