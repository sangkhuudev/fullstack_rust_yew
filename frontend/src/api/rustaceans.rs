use gloo_net::http::Request;
use gloo_net::Error;
use serde_json::json;
use serde::Deserialize;
use super::API_HOST;


#[derive(PartialEq, Deserialize, Clone)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}




pub async fn api_rustaceans(token: &String) -> Result<Vec<Rustacean>, Error> {
    let response = Request::get(&format!("{}/rustaceans", API_HOST))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await?;

    response.json::<Vec<Rustacean>>().await

}

pub async fn api_rustacean_create(
    token: &String,
    name: &String,
    email: &String,
) -> Result<Rustacean, Error> {
    let response = Request::post(&format!("{}/rustaceans", API_HOST))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({
            "name": name,
            "email": email,
        }))?
        .send()
        .await?;

    response.json::<Rustacean>().await

}