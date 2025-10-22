//! CodeSage CLI Binary Entry Point

use codesage_cli::run;
use codesage_core::Result;

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}
