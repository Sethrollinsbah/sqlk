use std::time::Instant;

use anyhow::Result;
use clap::Parser;
use sqlk::application::app::App;

#[tokio::main]
async fn main() -> Result<()> {
    let start_time = Instant::now();

    let args = sqlk::args::Args::parse();

    if args.env.exists() {
        dotenv::from_path(&args.env).ok();
    }

    let mut app = App::new(args)?;
    app.run(start_time).await?;

    Ok(())
}
