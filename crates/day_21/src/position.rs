#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Position(pub i32, pub i32);

impl Position {
    pub fn sub(&self, other: Position) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}
