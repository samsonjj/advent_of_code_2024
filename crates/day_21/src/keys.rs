use crate::position::Position;

pub trait Key: Sized {
    fn values() -> &'static [Self];
    // fn as_char(c: char) -> char;
    fn from_pos(p: Position) -> Option<Self>;
    fn as_pos(&self) -> Position;
    fn apply(&self, d: DirectionalKey) -> Option<Self>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NumericKey {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    A,
}

impl NumericKey {
    fn from_char(c: char) -> Self {
        match c {
            '0' => Self::_0,
            '1' => Self::_1,
            '2' => Self::_2,
            '3' => Self::_3,
            '4' => Self::_4,
            '5' => Self::_5,
            '6' => Self::_6,
            '7' => Self::_7,
            '8' => Self::_8,
            '9' => Self::_9,
            'A' => Self::A,
            _ => panic!("invalid char"),
        }
    }
}

impl Key for NumericKey {
    fn values() -> &'static [Self] {
        &[
            Self::_0,
            Self::_1,
            Self::_2,
            Self::_3,
            Self::_4,
            Self::_5,
            Self::_6,
            Self::_7,
            Self::_8,
            Self::_9,
            Self::A,
        ]
    }

    fn as_pos(&self) -> Position {
        use NumericKey::*;
        let p = match self {
            _0 => (3, 1),
            _1 => (2, 0),
            _2 => (2, 1),
            _3 => (2, 2),
            _4 => (1, 0),
            _5 => (1, 1),
            _6 => (1, 2),
            _7 => (0, 0),
            _8 => (0, 1),
            _9 => (0, 2),
            A => (3, 2),
        };
        Position(p.0, p.1)
    }
    fn from_pos(pos: Position) -> Option<Self> {
        use NumericKey::*;
        match (pos.0, pos.1) {
            (3, 1) => Some(_0),
            (2, 0) => Some(_1),
            (2, 1) => Some(_2),
            (2, 2) => Some(_3),
            (1, 0) => Some(_4),
            (1, 1) => Some(_5),
            (1, 2) => Some(_6),
            (0, 0) => Some(_7),
            (0, 1) => Some(_8),
            (0, 2) => Some(_9),
            (3, 2) => Some(A),
            _ => None,
        }
    }
    fn apply(&self, dirkey: DirectionalKey) -> Option<Self> {
        let pos = self.as_pos();
        let (d0, d1) = dirkey.as_delta();
        let new_pos = Position(pos.0 + d0, pos.1 + d1);
        Self::from_pos(new_pos)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DirectionalKey {
    Up,
    Down,
    Left,
    Right,
    A,
}

impl DirectionalKey {
    pub fn directional_values() -> &'static [Self] {
        &[Self::Up, Self::Down, Self::Left, Self::Right]
    }
    pub fn as_delta(&self) -> (i32, i32) {
        match self {
            DirectionalKey::Up => (-1, 0),
            DirectionalKey::Down => (1, 0),
            DirectionalKey::Left => (0, -1),
            DirectionalKey::Right => (0, 1),
            _ => panic!("A does not have a delta"),
        }
    }
}

impl Key for DirectionalKey {
    fn values() -> &'static [Self] {
        &[Self::Up, Self::Down, Self::Left, Self::Right, Self::A]
    }

    fn from_pos(p: Position) -> Option<Self> {
        let pos = (p.0, p.1);
        match pos {
            (0, 1) => Some(Self::Up),
            (0, 2) => Some(Self::A),
            (1, 0) => Some(Self::Left),
            (1, 1) => Some(Self::Down),
            (1, 2) => Some(Self::Right),
            _ => None,
        }
    }

    fn as_pos(&self) -> Position {
        let p = match self {
            Self::Up => (0, 1),
            Self::A => (0, 2),
            Self::Left => (1, 0),
            Self::Down => (1, 1),
            Self::Right => (1, 2),
        };
        Position(p.0, p.1)
    }

    fn apply(&self, dirkey: DirectionalKey) -> Option<Self> {
        let (d0, d1) = dirkey.as_delta();
        let pos = self.as_pos();
        let new_pos = Position(pos.0 + d0, pos.1 + d1);
        Self::from_pos(new_pos)
    }
}
