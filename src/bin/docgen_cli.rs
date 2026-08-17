use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    docgen_cli::run_cli().await
}
