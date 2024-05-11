#[derive(Clone, Debug)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub blog_name: String,
    pub blog_description: String,
    pub elsewhere: String,
}

impl NewUser {
    pub fn empty() -> Self {
        Self {
            email: String::new(),
            username: String::new(),
            nickname: String::new(),
            password: String::new(),
            blog_name: String::new(),
            blog_description: String::new(),
            elsewhere: String::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SignInfo {
    pub email: String,
    pub username: String,
    pub token: String,
}

impl SignInfo {
    pub fn empty() -> Self {
        Self {
            email: String::new(),
            username: String::new(),
            token: String::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UserNewToken {
    pub id: String,
    pub email: String,
    pub username: String,
    pub token: String,
}

impl UserNewToken {
    pub fn empty() -> Self {
        Self {
            id: String::new(),
            email: String::new(),
            username: String::new(),
            token: String::new(),
        }
    }

    pub fn is_logged_in(&self) -> bool {
        !self.token.is_empty()
    }
}

#[derive(Clone)]
pub struct CategoryNewType {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Clone)]
pub struct CategoryUserNewType {
    pub id: String,
    pub user_id: String,
    pub category_id: String,
}

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

pub struct ArticleType {
    pub id: String,
    pub user_id: String,
    pub category_id: String,
    pub subject: String,
    pub summary: String,
    pub slug: String,
    pub content: String,
    pub published: bool,
    pub top: bool,
    pub recommended: bool,
}

impl ArticleType {
    pub fn empty() -> Self {
        Self {
            id: String::new(),
            user_id: String::new(),
            category_id: String::new(),
            subject: String::new(),
            summary: String::new(),
            slug: String::new(),
            content: String::new(),
            published: false,
            top: false,
            recommended: false,
        }
    }
}
