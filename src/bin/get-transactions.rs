use luno::LunoClient;

fn main() {
    let key = String::from("LUNO_API_KEY");
    let secret = String::from("LUNO_API_SECRET");

    let client = LunoClient::new(key, secret);

    match client.get_transactions("ACCOUNT_ID", 0, 10) {
        Err(e) => eprintln!("{:?}", e),
        Ok(result) => {
            for txn in result.transactions.into_iter() {
                println!("{:?}", txn);
            }
        }
    }
}
