fn main() {
    match std::env::var("MY_SECRET") {
        Ok(secret) => {
            println!("secret is: {} bytes long", secret.len());
            println!("secret is: {}", secret);
        },
        Err(_) => println!("no secret"),
    }
}
