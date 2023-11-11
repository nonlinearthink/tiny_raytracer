#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::default()
    }

    pub fn u(&self) -> f32 {
        self.x
    }

    pub fn v(&self) -> f32 {
        self.y
    }
}

impl std::ops::Add<&Vector2> for Vector2 {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub<&Vector2> for Vector2 {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Mul<&Vector2> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl std::ops::Div<&Vector2> for Vector2 {
    type Output = Self;

    fn div(self, rhs: &Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}
