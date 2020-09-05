use luno::LunoClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LunoClient::new("LUNO_API_KEY", "LUNO_API_SECRET");

    println!(
        "{:?}",
        client.list_pending_transactions("ACCOUNT_ID").await?
    );
    Ok(())
}
