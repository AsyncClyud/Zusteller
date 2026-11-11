use crate::app::PostMan;
use std::sync::mpsc::{self};
use std::thread;

pub fn requesting(requ: &mut PostMan) {
    requ.response = String::new();

    let (req, resp) = mpsc::channel();
    let adress = requ.adress.clone();
    let body = requ.body.clone();
    let method = requ.method.clone();
    let client = requ.client.clone();

    requ.receiver = Some(resp);
    if requ.method == "GET"
        || requ.method == "POST"
        || requ.method == "PUT"
        || requ.method == "DELETE"
    {
        create_thread(adress, client, method, body, req);
    } else {
        let _ = req.send(Err(String::from("Invalid method!")));
    }
}

fn create_thread(
    adress: String,
    client: reqwest::Client,
    method: String,
    body: String,
    req: std::sync::mpsc::Sender<Result<String, String>>,
) {
    thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        if method == "GET" {
            let result = runtime
                .block_on(get(&client, adress))
                .map_err(|e| e.to_string());
            let _ = req.send(result);
        } else if method == "POST" {
            let result = runtime
                .block_on(post(&client, adress, body))
                .map_err(|e| e.to_string());
            let _ = req.send(result);
        } else if method == "PUT" {
            let result = runtime
                .block_on(put(&client, adress, body))
                .map_err(|e| e.to_string());
            let _ = req.send(result);
        } else if method == "DELETE" {
            let result = runtime
                .block_on(delete(&client, adress, body))
                .map_err(|e| e.to_string());
            let _ = req.send(result);
        }
    });
}

async fn get(client: &reqwest::Client, adress: String) -> Result<String, reqwest::Error> {
    let answer = client
        .get(adress)
        .header("User-Agent", "Zusteller/0.0.1")
        .send()
        .await?
        .text()
        .await?;
    return Ok(answer);
}

async fn post(
    client: &reqwest::Client,
    adress: String,
    body: String,
) -> Result<String, reqwest::Error> {
    let answer = client
        .post(adress)
        .body(body)
        .header("User-Agent", "Zusteller/0.0.1")
        .send()
        .await?
        .text()
        .await?;
    return Ok(answer);
}

async fn put(
    client: &reqwest::Client,
    adress: String,
    body: String,
) -> Result<String, reqwest::Error> {
    let answer = client
        .put(adress)
        .body(body)
        .header("User-Agent", "Zusteller/0.0.1")
        .send()
        .await?
        .text()
        .await?;
    return Ok(answer);
}

async fn delete(
    client: &reqwest::Client,
    adress: String,
    body: String,
) -> Result<String, reqwest::Error> {
    let answer = client
        .delete(adress)
        .header("User-Agent", "Zusteller/0.0.1")
        .body(body)
        .send()
        .await?
        .text()
        .await?;
    return Ok(answer);
}
