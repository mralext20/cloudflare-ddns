use reqwest::blocking::Client;

const SLEEP_FROM_NO_RESPONSE: u64 = 30; // sleep for half a minute if no response from server
const LOOP_TIMER: u64 = 60 * 15; // check every 15 minutes

fn main() {
    let client = Client::new();

    let env_keys = ["ZONE_ID", "RECORD_ID", "API_KEY", "RECORD_NAME"];
    for key in env_keys {
        match std::env::var(key) {
            Ok(val) => println!("{}: {}", key, val),
            Err(_) => {
                println!("Error: {} not found in environment variables", key);
                std::process::exit(1);
            }
        }
    }
    let api_key = std::env::var("API_KEY").unwrap();
    let zone_id = std::env::var("ZONE_ID").unwrap();
    let record_id = std::env::var("RECORD_ID").unwrap();
    let record_name = std::env::var("RECORD_NAME").unwrap();

    let mut curr_ip_addr = String::new();

    loop {
        let ip_addr = match client.get("https://api.ipify.org/").send() {
            Ok(response) => match response.text() {
                Ok(text) => text,
                Err(err) => {
                    println!(
                        "Error: Failed to get IP address: {}, trying again in {} seconds",
                        err, SLEEP_FROM_NO_RESPONSE
                    );
                    std::thread::sleep(std::time::Duration::from_secs(SLEEP_FROM_NO_RESPONSE));
                    continue;
                }
            },
            Err(err) => {
                println!(
                    "Error: Failed to get IP address: {}, trying again in {} seconds",
                    err, SLEEP_FROM_NO_RESPONSE
                );
                std::thread::sleep(std::time::Duration::from_secs(SLEEP_FROM_NO_RESPONSE));
                continue;
            }
        };
        if ip_addr != curr_ip_addr {
            println!("IP address changed to {}", ip_addr);
            curr_ip_addr = ip_addr.clone();

            match client
                .put(format!("https://api.cloudflare.com/client/v4/zones/{ZONE_ID}/dns_records/{RECORD_ID}",ZONE_ID=zone_id,RECORD_ID=record_id))
                .header("Authorization", format!("Bearer {}", api_key))
                .header("Content-Type", "application/json")
                .body(format!(
                    "{{\"type\":\"A\",\"name\":\"{}\",\"content\":\"{}\",\"ttl\":1,\"proxied\":true}}",
                    record_name,
                    ip_addr
                ))
                .send()
            {
                Ok(response) => match response.text() {
                    Ok(text) => println!("Response: {}", text),
                    Err(_) => println!("Error: Failed to get response text"),
                },
                Err(_) => println!("Error: Failed to send request"),
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(LOOP_TIMER));
    }
}
