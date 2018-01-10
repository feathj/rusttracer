use std::ops;

#[derive(Debug)]
pub struct Color {
    pub r:f64,
    pub g:f64,
    pub b:f64
}

impl Color {
    pub fn new(a_r:f64, a_g:f64, a_b:f64) -> Color {
        return Color {
            r: a_r,
            g: a_g,
            b: a_b
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other:&Color) -> bool {
        return self.r.round() == other.r.round()
            && self.g.round() == other.g.round()
            && self.b.round() == other.b.round();
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs:Color) -> Color {
        return Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b
        }
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs:Color) -> Color {
        return Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b
        }
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs:f64) -> Color {
        return Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs
        }
    }
}
