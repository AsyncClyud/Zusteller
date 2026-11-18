use crate::ui::app;
use reqwest::header::HeaderMap;
use rust_i18n::t;

use std::{
    sync::mpsc::{self},
    time::Duration,
};

pub fn requesting(requ: &mut app::PostMan) {
    requ.response = t!("request.waiting").to_string();

    let (req, resp) = mpsc::channel();
    let adress = requ.adress.clone();
    let headers = requ.headermap.clone();
    let body = requ.body.clone();
    let method = requ.method.clone();
    let client = requ.client.clone();

    requ.receiver = Some(resp);
    create_thread(
        adress,
        client,
        requ.runtime.handle(),
        method,
        headers,
        body,
        req,
    );
}

fn create_thread(
    adress: String,
    client: reqwest::Client,
    runtime: &tokio::runtime::Handle,
    method: String,
    headers: HeaderMap,
    body: String,
    req: std::sync::mpsc::Sender<Result<String, String>>,
) {
    runtime.spawn(async move {
        let result = match method.as_str() {
            "GET" => get(&client, adress, headers)
                .await
                .map_err(|e| e.to_string()),

            "POST" => post(&client, adress, headers, body)
                .await
                .map_err(|e| e.to_string()),

            "PUT" => put(&client, adress, headers, body)
                .await
                .map_err(|e| e.to_string()),

            "PATCH" => patch(&client, adress, headers, body)
                .await
                .map_err(|e| e.to_string()),

            "DELETE" => delete(&client, adress, headers, body)
                .await
                .map_err(|e| e.to_string()),

            _ => Err(t!("errors.invalid_method").to_string().into()),
        };

        let _ = req.send(result);
    });
}

fn format_request_error(error: reqwest::Error) -> String {
    if error.is_timeout() {
        t!("errors.timeout").to_string().into()
    } else {
        t!("errors.timeout", error = error).to_string()
    }
}

async fn get(
    client: &reqwest::Client,
    adress: String,
    headers: HeaderMap,
) -> Result<String, String> {
    let response = client
        .get(adress)
        .timeout(Duration::from_secs(30))
        .headers(headers)
        .send()
        .await
        .map_err(format_request_error)?;
    let body = response.text().await.map_err(format_request_error)?;
    Ok(body)
}

async fn post(
    client: &reqwest::Client,
    adress: String,
    headers: HeaderMap,
    body: String,
) -> Result<String, String> {
    let response = client
        .post(adress)
        .body(body)
        .timeout(Duration::from_secs(30))
        .headers(headers)
        .send()
        .await
        .map_err(format_request_error)?;
    let body = response.text().await.map_err(format_request_error)?;
    Ok(body)
}

async fn put(
    client: &reqwest::Client,
    adress: String,
    headers: HeaderMap,
    body: String,
) -> Result<String, String> {
    let response = client
        .put(adress)
        .body(body)
        .timeout(Duration::from_secs(30))
        .headers(headers)
        .send()
        .await
        .map_err(format_request_error)?;
    let body = response.text().await.map_err(format_request_error)?;
    Ok(body)
}

async fn patch(
    client: &reqwest::Client,
    adress: String,
    headers: HeaderMap,
    body: String,
) -> Result<String, String> {
    let response = client
        .patch(adress)
        .body(body)
        .timeout(Duration::from_secs(30))
        .headers(headers)
        .send()
        .await
        .map_err(format_request_error)?;
    let body = response.text().await.map_err(format_request_error)?;
    Ok(body)
}

async fn delete(
    client: &reqwest::Client,
    adress: String,
    headers: HeaderMap,
    body: String,
) -> Result<String, String> {
    let response = client
        .delete(adress)
        .body(body)
        .timeout(Duration::from_secs(30))
        .headers(headers)
        .send()
        .await
        .map_err(format_request_error)?;
    let body = response.text().await.map_err(format_request_error)?;
    Ok(body)
}
