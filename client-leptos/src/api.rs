/// Get URL from a backend route; we can swap out the base URL depending on
/// the environment, using our production backend URL for prod.
pub fn get_url(route: &str) -> String {
    format!("http://localhost:8000{}", route)
}
