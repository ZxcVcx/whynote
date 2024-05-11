use once_cell::sync::Lazy;
use std::collections::HashMap;

use serde::Deserialize;
#[derive(Deserialize)]
pub struct Config {
    pub site: Site,
    pub gql: Gql,
    pub comment: Comment,
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

#[derive(Deserialize)]
pub struct Comment {
    pub repo: String,
    pub repoid: String,
    pub category: String,
    pub categoryid: String,
    pub mapping: String,
    pub strict: String,
    pub reactionsenabled: String,
    pub emitmetadata: String,
    pub inputposition: String,
    pub theme: String,
    pub lang: String,
    pub loading: String,
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

    cfg.insert("COMMENT_REPO", config.comment.repo);
    cfg.insert("COMMENT_REPO_ID", config.comment.repoid);
    cfg.insert("COMMENT_CATEGORY", config.comment.category);
    cfg.insert("COMMENT_CATEGORY_ID", config.comment.categoryid);
    cfg.insert("COMMENT_MAPPING", config.comment.mapping);
    cfg.insert("COMMENT_STRICT", config.comment.strict);
    cfg.insert("COMMENT_REACTIONS_ENABLED", config.comment.reactionsenabled);
    cfg.insert("COMMENT_EMIT_METADATA", config.comment.emitmetadata);
    cfg.insert("COMMENT_INPUT_POSITION", config.comment.inputposition);
    cfg.insert("COMMENT_THEME", config.comment.theme);
    cfg.insert("COMMENT_LANG", config.comment.lang);
    cfg.insert("COMMENT_LOADING", config.comment.loading);

    cfg
});
