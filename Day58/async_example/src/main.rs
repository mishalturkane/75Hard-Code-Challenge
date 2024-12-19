use tokio::time::{sleep, Duration};

async fn fetch_data_from_server_1() -> String {
    sleep(Duration::from_secs(2)).await; // Simulates a 2-second delay
    "Data from server 1".to_string()
}

async fn fetch_data_from_server_2() -> String {
    sleep(Duration::from_secs(2)).await; // Simulates a 3-second delay
    "Data from server 2".to_string()
}

#[tokio::main]
async fn main() {
    println!("Fetching data...");

    // Run both tasks concurrently
    let (result1, result2) = tokio::join!(
        fetch_data_from_server_1(),
        fetch_data_from_server_2(),
    );

    println!("Received: {} and {}", result1, result2);
}
