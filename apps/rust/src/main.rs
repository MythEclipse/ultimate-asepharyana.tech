#![doc = "Logging Setup"]
use rustexpress::bootstrap::Application;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    // Application setup and server logic is now encapsulated in `startup.rs`
    // This makes the main function clean and the app easier to integration test.
    let app = Application::build().await?;
    app.run().await?;
    
    Ok(())
}
