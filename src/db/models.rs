#[derive(Queryable)]
pub struct Request {
    pub id: i32,
    pub domain: String,
    pub request_date: String,
    pub response: String,
    pub throttled: bool,
}