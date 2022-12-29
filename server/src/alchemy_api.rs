use rocket::serde::{Deserialize, Serialize, json::json};
use reqwest::{Client, header};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TokenBalancesApiResult {
    pub address: String,
    pub tokenBalances: Vec<TokenBalance>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TokenBalance {
    pub contractAddress: String,
    pub tokenBalance: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TokenInfoApiResult {
    pub decimals: i32,
    pub logo: Option<String>,
    pub name: String,
    pub symbol: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NftInfoApiResult {}

/// Make an RPC POST request to a given endpoint, parse and return the JSON response
async fn make_post_request<T>(api_url: &String, endpoint: String, params: String) -> T where T: for<'a> Deserialize<'a> {
    let data = json!({
        "jsonrpc": "2.0",
        "method": endpoint,
        "headers": {
            "Content-Type": "application/json",
        },
        "params": [params]
    });

    let client = Client::new();
    let res = client
        .post(api_url)
        .header(header::CONTENT_TYPE, "application/json")
        .json(&data)
        .send()
        .await;

    let result = res.unwrap();
    let result = result.json::<serde_json::Value>().await.unwrap();
    let result: T = serde_json::from_value(result["result"].clone()).unwrap();
    result
}

/// Make an HTTP GET request to a given endpoint, parse and return the JSON response
async fn make_get_request<T>(api_url: &String, endpoint: String, params: (String, String)) -> T where T: for<'a> Deserialize<'a> {
    let client = Client::new();
    let res = client
        .get(api_url)
        .query(&[params])
        .send()
        .await;

    let result = res.unwrap();
    let result = result.json::<serde_json::Value>().await.unwrap();
    let result: T = serde_json::from_value(result).unwrap();
    result
}

/// Get a list of tokens owned by a given address
pub async fn get_balances(api_url: &String, wallet_address: String) -> TokenBalancesApiResult {
    let result: TokenBalancesApiResult = make_post_request(
        &api_url,
        "alchemy_getTokenBalances".to_string(),
        format!("{}", wallet_address),
    ).await;
    result
}

/// Get the metadata for a given token by its contract address
pub async fn get_tokens_metadata(api_url: &String, contract_address: &String) -> TokenInfoApiResult {
    let result: TokenInfoApiResult = make_post_request(
        &api_url,
        "alchemy_getTokenMetadata".to_string(),
        format!("{}", contract_address),
    ).await;
    result
}

/// Get a list of NFTs owned by a given address
pub async fn get_nfts(api_url: &String, wallet_address: String) -> NftInfoApiResult {
    let result: NftInfoApiResult = make_get_request(
        &api_url,
        "getNFTs".to_string(),
        ("owner".to_string(), wallet_address),
    ).await;
    result
}