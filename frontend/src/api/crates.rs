use gloo_net::http::Request;
use gloo_net::Error;
use serde_json::json;
use serde::Deserialize;
use super::API_HOST;


#[derive(PartialEq, Deserialize, Clone)]
pub struct Crate {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub rustacean_id: i32,
    pub version: String,
    pub description: Option<String>,
    pub created_at: String,
}




pub async fn api_crates(token: &str) -> Result<Vec<Crate>, Error> {
    let response = Request::get(&format!("{}/crates", API_HOST))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await?;

    response.json::<Vec<Crate>>().await

}


pub async fn api_crate_show(token: &str, id: i32) -> Result<Crate, Error> {
    let response = Request::get(&format!("{}/crates/{}", API_HOST, id))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await?;

    response.json::<Crate>().await

}
pub async fn api_crate_create(
    token: &String,
    name: &String,
    code: &String,
    rustacean_id: i32,
    version: &String,
    description: &String,
) -> Result<Crate, Error> {
    let response = Request::post(&format!("{}/crates", API_HOST))
        .header("Authorization", &format!("Bearer {}", token))
        .header("accept", "application/json")
        .json(&json!({
            "name": name,
            "code": code,
            "rustacean_id": rustacean_id,
            "version": version,
            "description": description
        }))?
        .send()
        .await?;

    response.json::<Crate>().await

}

pub async fn api_crate_update(
    token: &String,
    id: i32,
    name: &String,
    code: &String,
    rustacean_id: i32,
    version: &String,
    description: &String,
) -> Result<Crate, Error> {
    let response = Request::put(&format!("{}/crates/{}", API_HOST, id))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({
            "name": name,
            "code": code,
            "rustacean_id": rustacean_id,
            "version": version,
            "description": description
        }))?
        .send()
        .await?;

    response.json::<Crate>().await

}

pub async fn api_crate_delete(
    token: &String,
    id: i32,
) -> Result<(), Error> {
    let _ = Request::delete(&format!("{}/crates/{}", API_HOST, id))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await?;

    Ok(())
}