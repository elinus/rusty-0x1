use shared_utils::User;

pub struct Database;

impl Database {
    pub fn new() -> Self {
        Database
    }

    pub async fn get_user(&self, id: u64) -> Option<User> {
        // Database logic here
        Some(User {
            id,
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        })
    }
}
