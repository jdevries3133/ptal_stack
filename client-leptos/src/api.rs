use reqwest::{Client, Method, Request, RequestBuilder, Url};

use super::auth::Auth;

pub struct Api {
    auth: Auth,
    client: Client,
}

impl Api {
    pub fn new(auth: Auth) -> Self {
        Api {
            auth,
            client: Client::new(),
        }
    }
    /// Initialize a request to the backend, including the authorization
    /// header
    pub fn get_req(&self, method: Method, url: &str) -> RequestBuilder {
        todo!()
        // let url = Url::parse(url).expect("url is valid");
        // let req = RequestBuilder::from_parts(self.client.to_owned(), Request::new(method, url))
        //     .header("content-type", "application/json");

        // self.auth.add_auth(req)
    }
}

/// Get URL from a backend route; we can swap out the base URL depending on
/// the environment, using our production backend URL for prod.
pub fn get_url(route: &str) -> String {
    format!("http://localhost:8000{}", route)
}
