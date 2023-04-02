pub mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct User {
        pub id: i32,
        pub username: String,
        pub email: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct LoginPayload {
        /// Can be username or email
        pub identifier: String,
        pub password: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct LoginResponse {
        pub session_token: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct RegisterPayload {
        pub username: String,
        pub email: String,
        pub password: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Profile {
        pub user: User,
        pub dog_photo_of_the_day_href: String,
    }
}
