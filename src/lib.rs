use anyhow::Context;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use spin_sdk::http::body::IncomingBodyExt;
use spin_sdk::http::{
    EmptyBody, FullBody, IntoResponse, Method, Request, Response, StatusCode,
    send,
};
use spin_sdk::http_service;

const RANDOM_DATA_API: &str = "https://random-data-api.fermyon.app";

#[derive(Debug, Deserialize)]
struct FactItem {
    fact: String,
}

#[derive(Debug, Serialize)]
struct AggregatedFacts {
    animal_fact: String,
    physics_fact: String,
}

async fn fetch_fact(url: &str) -> anyhow::Result<String> {
    let req = Request::builder()
        .method(Method::GET)
        .uri(url)
        .body(EmptyBody::new())
        .context("failed to build request")?;

    let res: Response = send(req).await.context("outbound request failed")?;
    if res.status() != StatusCode::OK {
        anyhow::bail!("unexpected status: {}", res.status());
    }

    let body = res.into_body().bytes().await?;
    let item: FactItem = serde_json::from_slice(&body)
        .context("failed to parse upstream fact")?;
    Ok(item.fact)
}

#[http_service]
async fn handle_aggregate_json(_req: Request) -> anyhow::Result<impl IntoResponse> {
    let animals_url = format!("{RANDOM_DATA_API}/animals/json");
    let physics_url = format!("{RANDOM_DATA_API}/physics/json");

    let (animal_res, physics_res) = futures::join!(
        fetch_fact(&animals_url),
        fetch_fact(&physics_url)
    );

    let animal_fact = animal_res?;
    let physics_fact = physics_res?;

    let aggregated = AggregatedFacts {
        animal_fact,
        physics_fact,
    };

    let body = serde_json::to_vec(&aggregated)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(FullBody::new(Bytes::from(body)))
        .unwrap())
}
