use luno::LunoClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let client = LunoClient::new("LUNO_API_KEY", "LUNO_API_SECRET");

	Ok(println!("{:?}", client.cancel_order("ORDER_ID").await?))
}
