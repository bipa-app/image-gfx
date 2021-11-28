#[derive(Debug, Clone, Copy)]
pub struct Rect {
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

impl Rect {
    pub fn new(left: u32, top: u32, width: u32, height: u32) -> Rect {
        Self {
            left,
            top,
            width,
            height,
        }
    }

    pub fn left(&self) -> u32 {
        self.left
    }

    pub fn right(&self) -> u32 {
        self.left + self.width
    }

    pub fn top(&self) -> u32 {
        self.top
    }

    pub fn bottom(&self) -> u32 {
        self.top + self.height
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    center_x: u32,
    center_y: u32,
    radius: u32,
}

impl Circle {
    pub fn new(center: (u32, u32), radius: u32) -> Self {
        Self {
            center_x: center.0,
            center_y: center.1,
            radius,
        }
    }

    pub fn center(&self) -> (u32, u32) {
        (self.center_x, self.center_y)
    }

    pub fn radius(&self) -> u32 {
        self.radius
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Angle {
    Degrees(f64),
    Radians(f64),
}

impl Angle {
    pub fn to_degrees(&self) -> f64 {
        match self {
            Angle::Degrees(d) => *d,
            Angle::Radians(r) => r.to_degrees(),
        }
    }

    pub fn to_radians(&self) -> f64 {
        match self {
            Angle::Degrees(d) => d.to_radians(),
            Angle::Radians(r) => *r,
        }
    }
}
