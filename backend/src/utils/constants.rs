use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::collections::HashMap;

// async-graphql result type
pub type GqlResult<T> = std::result::Result<T, async_graphql::Error>;

// datetime format
pub const DT_F: &str = "%Y-%m-%d %H:%M:%S%Z";

// https://docs.rs/once_cell/latest/once_cell/sync/struct.Lazy.html
pub static CFG: Lazy<HashMap<&'static str, String>> = Lazy::new(|| {
    dotenv().ok();

    let mut map = HashMap::new();

    map.insert(
        "ADDR",
        dotenv::var("ADDR").expect("Expected ADDR to be set in env!"),
    );
    map.insert(
        "PORT",
        dotenv::var("PORT").expect("Expected PORT to be set in env!"),
    );

    map.insert(
        "SITE_KEY",
        dotenv::var("SITE_KEY").expect("Expected SITE_KEY to be set in env!"),
    );
    map.insert(
        "CLAIM_EXP",
        dotenv::var("CLAIM_EXP").expect("Expected CLAIM_EXP to be set in env!"),
    );

    map.insert(
        "GQL_PATH",
        dotenv::var("GQL_PATH").expect("Expected GQL_URI to be set in env!"),
    );
    map.insert(
        "GIQL_PATH",
        dotenv::var("GIQL_PATH").expect("Expected GIQL_URI to be set in env!"),
    );
    map.insert(
        "GQL_VER",
        dotenv::var("GQL_VER").expect("Expected GQL_VER to be set in env!"),
    );
    map.insert(
        "GIQL_VER",
        dotenv::var("GIQL_VER").expect("Expected GIQL_VER to be set in env!"),
    );

    map.insert(
        "MONGODB_URI",
        dotenv::var("MONGODB_URI").expect("Expected MONGODB_URI to be set in env!"),
    );
    map.insert(
        "MONGODB_NAME",
        dotenv::var("MONGODB_NAME").expect("Expected MONGODB_NAME to be set in env!"),
    );

    map
});
