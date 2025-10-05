use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub providers: Vec<Provider>,
    pub default_provider: String,
}
#[derive(Deserialize, Debug)]
pub struct Provider {
    pub name: String,
    pub api_key: String,
    pub endpoint: String,
}