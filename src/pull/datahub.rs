use serde::Deserialize;
use anyhow::{Result, bail, anyhow};

#[derive(Deserialize, Default, Debug)]
pub struct DatahubCountry {
    #[serde(alias = "Code")]
    pub code: String,
    #[serde(alias = "Name")]
    pub name: String
}

impl DatahubCountry {
    pub fn pull(url: &str) -> Result<Vec<Self>> {
        println!("Pull ISO 3166-1 country data from {}...", url);
    
        let resp = match reqwest::blocking::get(url) {
            Ok(r) => r,
            Err(e) => bail!("Failed to read link ({}): {}", url, e),
        };
    
        let html = match resp.text() {
            Ok(s) => s,
            Err(e) => bail!("Failed to read http response from host: {}", e),
        };
        
        serde_json::from_str::<Vec<Self>>(&html)
            .map_err(|e| anyhow!("Failed to read countries as JSON: {}", e))
    }
}

#[derive(Deserialize, Default, Debug)]
pub struct DatahubCurrency {
    #[serde(alias = "Entity")]
    pub exonym: String,
    #[serde(alias = "AlphabeticCode")]
    pub iso_4217: Option<String>,
    #[serde(alias = "Currency")]
    pub name: Option<String>
}

impl DatahubCurrency {
    pub fn pull(url: &str) -> Result<Vec<Self>> {
        println!("Pull currency data from {}...", url);
    
        let resp = match reqwest::blocking::get(url) {
            Ok(r) => r,
            Err(e) => bail!("Failed to read link ({}): {}", url, e),
        };
    
        let html = match resp.text() {
            Ok(s) => s,
            Err(e) => bail!("Failed to read http response from host: {}", e),
        };
        
        serde_json::from_str::<Vec<Self>>(&html)
            .map_err(|e| anyhow!("Failed to read currencies as JSON: {}", e))
    }
}