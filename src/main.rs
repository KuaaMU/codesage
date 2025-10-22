// Main entry point - delegates to CLI crate
#[tokio::main]
async fn main() -> codesage_core::Result<()> {
    codesage_cli::run().await
}
