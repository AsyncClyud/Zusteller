use crate::ui::app;
use reqwest::header::HeaderMap;

use std::sync::mpsc::{self};
use std::thread;

pub fn requesting(requ: &mut app::PostMan) {
    requ.response = String::from("Waiting for response...");

    let (req, resp) = mpsc::channel();
    let adress = requ.adress.clone();
    let headers = requ.headermap.clone();
    let body = requ.body.clone();
    let method = requ.method.clone();
    let client = requ.client.clone();

    requ.receiver = Some(resp);
    create_thread(adress, client, method, headers, body, req);
}

fn create_thread(
    adress: String,
    client: reqwest::Client,
    method: String,
    headers: HeaderMap,
    body: String,
    req: std::sync::mpsc::Sender<Result<String, String>>,
) {
    thread::spawn(move || {
        let result = match tokio::runtime::Runtime::new() {
            Ok(runtime) => match method.as_str() {
                "GET" => runtime
                    .block_on(get(&client, adress, headers))
                    .map_err(|e| e.to_string()),
                "POST" => runtime
                    .block_on(post(&client, adress, headers, body))
                    .map_err(|e| e.to_string()),
                "PUT" => runtime
                    .block_on(put(&client, adress, headers, body))
                    .map_err(|e| e.to_string()),
                "PATCH" => runtime
                    .block_on(patch(&client, adress, headers, body))
                    .map_err(|e| e.to_string()),
                "DELETE" => runtime
                    .block_on(delete(&client, adress, headers, body))
                    .map_err(|e| e.to_string()),
                _ => Err(String::from("Invalid method!")),
            },
            Err(error) => Err(format!("Runtime error: {error}")),
        };
        let _ = req.send(result);
    });
}

async fn get(
    client: &reqwest::Client,
    adress: String,
    headers: HeaderMap,
) -> Result<String, reqwest::Error> {
    let answer = client
        .get(adress)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;
    return Ok(answer);
}

async fn post(
    client: &reqwest::Client,
    adress: String,
    headers: HeaderMap,
    body: String,
) -> Result<String, reqwest::Error> {
    let answer = client
        .post(adress)
        .body(body)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;
    return Ok(answer);
}

async fn put(
    client: &reqwest::Client,
    adress: String,
    headers: HeaderMap,
    body: String,
) -> Result<String, reqwest::Error> {
    let answer = client
        .put(adress)
        .body(body)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;
    return Ok(answer);
}

async fn patch(
    client: &reqwest::Client,
    adress: String,
    headers: HeaderMap,
    body: String,
) -> Result<String, reqwest::Error> {
    let answer = client
        .patch(adress)
        .body(body)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;
    return Ok(answer);
}

async fn delete(
    client: &reqwest::Client,
    adress: String,
    headers: HeaderMap,
    body: String,
) -> Result<String, reqwest::Error> {
    let answer = client
        .delete(adress)
        .headers(headers)
        .body(body)
        .send()
        .await?
        .text()
        .await?;
    return Ok(answer);
}
