use crate::go::state::stone::Stone;

#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    pub x: i8,
    pub y: i8,
    pub stone: Option<Stone>,
    pub territory_id: Option<i8>
}
