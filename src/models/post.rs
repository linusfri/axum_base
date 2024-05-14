use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Post {
    id: i32,
    title: String,
    body: String,
    published: bool,
}

impl Post {
    pub fn fetch_all() -> Vec<Post> {
        vec![
            Post {
                id: 1,
                title: "First post".to_string(),
                body: "This is the first post".to_string(),
                published: true,
            },
            Post {
                id: 2,
                title: "Second post".to_string(),
                body: "This is the second post".to_string(),
                published: false,
            },
        ]
    }
}