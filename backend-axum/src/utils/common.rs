// Generate friendly slug from the given string
pub async fn slugify(str: &str) -> String {
    // https://lib.rs/crates/deunicode
    use deunicode::deunicode_with_tofu;

    let slug = deunicode_with_tofu(str.trim(), "-")
        .to_lowercase()
        .replace(" ", "-")
        .replace("[", "-")
        .replace("]", "-")
        .replace("\"", "-")
        .replace("/", "-")
        .replace("?", "-");

    slug
}
