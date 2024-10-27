#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    pub player_number: i8,
    pub chain_id: i8,
    pub territory_id: Option<i8>
}
