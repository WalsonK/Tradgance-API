#[derive(Debug)]
pub enum Direction {
    Buy = 0,
    Sell = 1
}

impl From<i32> for Direction {
    fn from(item: i32) -> Self {
        match item {
            0 => Self::Buy,
            1 => Self::Sell,
            _ => unreachable!()
        }
    }
}

impl From<Direction> for i32 {
    fn from(item: Direction) -> Self {
        item as i32
    }
}