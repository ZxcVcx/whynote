use once_cell::sync::Lazy;
use std::collections::HashMap;

use serde::Deserialize;
#[derive(Deserialize)]
pub struct Config {
    pub site: Site,
    pub gql: Gql,
}

#[derive(Deserialize)]
pub struct Site {
    pub title: String,
}

#[derive(Deserialize)]
pub struct Gql {
    pub protocol: String,
    pub host: String,
    pub port: String,
    pub path: String,
}

// Import the `window.alert` function from the Web.
pub static CFG: Lazy<HashMap<&'static str, String>> = Lazy::new(|| {
    let cfg_str = include_str!("../../build_prop.toml");
    let config: Config = toml::from_str(cfg_str).unwrap();
    let mut cfg = HashMap::new();

    cfg.insert("SITE_TITLE", config.site.title);
    cfg.insert("GQL_PROTOCOL", config.gql.protocol);
    cfg.insert("GQL_HOST", config.gql.host);
    cfg.insert("GQL_PORT", config.gql.port);
    cfg.insert("GQL_PATH", config.gql.path);

    cfg
});
