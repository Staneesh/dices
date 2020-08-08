mod ws;

use std::error::Error;
use std::env;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:2137".to_string());

    ws::listen(addr).await?;

    Ok(())
}
