use tokio::time::{sleep, Duration};

async fn fetch_data() -> String {
    // Simulate a network request by sleeping for 2 seconds.
    sleep(Duration::from_secs(2)).await;
    "Data fetched!".to_string()
}

async fn process_data() {
    println!("Starting data fetch...");
    
    // Await the result of the fetch_data function.
    let data = fetch_data().await;
    
    println!("Processing: {}", data);
}

#[tokio::main]
async fn main() {
    // Call the async function inside the main async function.
    process_data().await;
}
