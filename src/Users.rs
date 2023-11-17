#[derive(Debug)]
pub struct User{
    pub name: String,
    pub id: u64,
    pub active: bool,
    pub charge_history: Vec<u32>,
    pub current_checkouts: Vec<u32>,
}