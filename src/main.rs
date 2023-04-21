use reqwest::blocking::Client;
use serde_json::{json, Value};

fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();

    let data = json!({
        "project_id": "p1",
        "company": "aqual_equal",
        "sensor_id": "x1234",
        "start_datetime": "10-04-2023 2:09:20",
        "end_datetime": "11-06-2023 2:04:50"
    });

    let response = client
        .post("https://sentry1.gcp-uscentral1.cudos.org:36657/")
        .header("Content-Type", "application/json")
        .body(data.to_string())
        .send()?;

    let response_text = response.text()?;
    let response_json: Value = serde_json::from_str(&response_text)?;

    println!("{:#?}", response_json);

    Ok(())
}
