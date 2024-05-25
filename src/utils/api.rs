use reqwest::{Error, header};
use crate::utils::scheme::{
    ChileanIndicators,
    ClpUsdHistory,
    CriptoIndicator,
    CriptoUsdHistory
};

pub async fn get_chilean_indicators() -> Result<ChileanIndicators, Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://mindicador.cl/api")
        .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.102 Safari/537.36")
        .send()
        .await?
        .json::<ChileanIndicators>()
        .await?;
    Ok(response)
}


pub async fn get_crypto_indicators() -> Result<Vec<CriptoIndicator>, Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&order=market_cap_desc&per_page=100&page=1&sparkline=false&locale=en")
        .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.102 Safari/537.36")
        .send()
        .await?
        .json::<Vec<CriptoIndicator>>()
        .await?;
    Ok(response)
}


pub async fn get_btc_usd_comp() -> Result<CriptoUsdHistory, Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.coingecko.com/api/v3/coins/bitcoin/market_chart?vs_currency=usd&days=30&interval=daily")
        .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.102 Safari/537.36")
        .send()
        .await?
        .json::<CriptoUsdHistory>()
        .await?;
    Ok(response)
}


pub async fn get_xmr_usd_comp() -> Result<CriptoUsdHistory, Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.coingecko.com/api/v3/coins/monero/market_chart?vs_currency=usd&days=30&interval=daily")
        .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.102 Safari/537.36")
        .send()
        .await?
        .json::<CriptoUsdHistory>()
        .await?;
    Ok(response)
}


pub async fn get_clp_usd_comp() -> Result<ClpUsdHistory, Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://mindicador.cl/api/dolar")
        .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.102 Safari/537.36")
        .send()
        .await?
        .json::<ClpUsdHistory>()
        .await?;
    Ok(response)
}
