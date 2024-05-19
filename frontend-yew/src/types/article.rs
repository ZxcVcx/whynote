
#[derive(Clone)]
pub struct ArticleNewType {
    pub user_id: String,
    pub category_id: String,
    pub subject: String,
    pub summary: String,
    pub content: String,
    pub published: bool,
    pub top: bool,
    pub recommended: bool,
}

impl ArticleNewType {
    pub fn empty() -> Self {
        Self {
            user_id: String::new(),
            category_id: String::new(),
            subject: String::new(),
            summary: String::new(),
            content: String::new(),
            published: false,
            top: false,
            recommended: false,
        }
    }
}

// #[derive(Clone)]
// pub struct ArticleNewStaticType {
//     pub user_id: &str,
//     pub category_id: &str,
//     pub subject: &str,
//     pub summary: &str,
//     pub content: &str,
//     pub published: bool,
//     pub top: bool,
//     pub recommended: bool,
// }
