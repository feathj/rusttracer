use color::Color;

#[derive(Debug)]
pub struct Surface {
    pub ambient: Color,
    pub diffuse: Color,
    pub specular: Color,
    pub shiny: f64,
    pub deflect: f64
}

impl Surface {
    pub fn new() -> Surface {
        return Surface {
            ambient: Color::new(1.0, 1.0, 1.0),
            diffuse: Color::new(1.0, 1.0, 1.0),
            specular: Color::new(1.0, 1.0, 1.0),
            shiny: 50.0,
            reflect: 0.0
        }
    }
}

impl PartialEq for Surface {
    fn eq(&self, other:&Surface) -> bool {
        return self.ambient == other.ambient
            && self.diffuse == other.diffuse
            && self.specular == other.specular
            && self.shiny == other.shiny
            && self.reflect == other.reflect
    }
}
