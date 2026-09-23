
    use std::fmt;

    #[derive(Default, Clone, Copy, Hash, PartialEq, Eq, Debug)]
    pub struct Vec2 {
        pub x: i32,
        pub y: i32,
    }

    impl Vec2 {
        #[allow(dead_code)]
        pub const fn new(x: i32, y: i32) -> Self {
            Vec2 { x, y }
        }

        pub fn broadcast(v: i32) -> Self {
            Vec2 { x: v, y: v }
        }
    }

    impl std::ops::Add<Vec2> for Vec2 {
        type Output = Vec2;
        fn add(self, rhs: Vec2) -> Self::Output {
            Vec2 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl std::ops::Sub<Vec2> for Vec2 {
        type Output = Vec2;
        fn sub(self, rhs: Vec2) -> Self::Output {
            Vec2 {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }

    impl std::ops::Mul<Vec2> for Vec2 {
        type Output = Vec2;
        fn mul(self, rhs: Vec2) -> Self::Output {
            Vec2 {
                x: self.x * rhs.x,
                y: self.y * rhs.y,
            }
        }
    }

    impl std::ops::Div<Vec2> for Vec2 {
        type Output = Vec2;
        fn div(self, rhs: Vec2) -> Self::Output {
            Vec2 {
                x: self.x / rhs.x,
                y: self.y / rhs.y,
            }
        }
    }

    impl fmt::Display for Vec2 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({} {})", self.x, self.y)
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

    impl From<Direction> for Vec2 {
        fn from(value: Direction) -> Vec2 {
            match value.into() {
                Direction::Left => Vec2 { x: -1, y: 0 },
                Direction::Right => Vec2 { x: 1, y: 0 },
                Direction::Up => Vec2 { x: 0, y: -1 },
                Direction::Down => Vec2 { x: 0, y: 1 },
            }
        }
    }

    impl From<&Direction> for Vec2 {
        fn from(value: &Direction) -> Vec2 {
            match value {
                Direction::Left => Vec2 { x: -1, y: 0 },
                Direction::Right => Vec2 { x: 1, y: 0 },
                Direction::Up => Vec2 { x: 0, y: -1 },
                Direction::Down => Vec2 { x: 0, y: 1 },
            }
        }
    }

