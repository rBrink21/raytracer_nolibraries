#[derive(Copy, Clone)]
pub struct Color {
    pub(crate) red: f32,
    pub(crate) green: f32,
    pub(crate) blue: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }
    pub fn from_red() -> Self {
        Self::new(255.0, 0.0, 0.0)
    }
    pub fn from_green() -> Self {
        Self::new(0.0, 255.0, 0.0)
    }
    pub fn from_blue() -> Self {
        Self::new(0.0, 0.0, 255.0)
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
    fn copy(color: &Color) -> Self {
        Self::new(color.red, color.green, color.blue)
    }
    fn clone(&self) -> Self {
        Color::copy(self)
    }

    fn subtract(&mut self, color: &Color) {
        self.green -= color.green;
        self.red -= color.red;
        self.blue -= color.blue;
    }

    fn add(&mut self, color: &Color) {
        self.green += color.green;
        self.red += color.red;
        self.blue += color.blue;
    }

    fn haramard_product(&mut self, color: &Color) {
        self.green *= color.green;
        self.red *= color.red;
        self.blue *= color.blue;
    }
}

#[cfg(test)]
mod color_tests {
    use crate::color::Color;

    #[test]
    fn test_colors() {
        let a = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(a.red, -0.5);
        assert_eq!(a.green, 0.4);
        assert_eq!(a.blue, 1.7);
    }

    #[test]
    fn test_haramard_product() {
        let mut a = Color::new(1.0, 1.0, 1.0);

        let b = Color::new(5.0, 5.0, 5.0);
        a.haramard_product(&b);
        assert_eq!(a.red, 5.0);
        assert_eq!(a.green, 5.0);
        assert_eq!(a.blue, 5.0);
    }
}
