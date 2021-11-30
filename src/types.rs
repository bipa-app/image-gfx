use std::f64::consts::PI;

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

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
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
    pub fn normalize(&mut self) {
        *self = self.normalized()
    }

    pub fn normalized(&self) -> Self {
        match self {
            Angle::Degrees(d) => {
                let d = *d;

                if 0f64 <= d && d <= 360f64 {
                    Angle::Degrees(d)
                } else {
                    let dd = d % 360f64;
                    Angle::Degrees(if dd < 0f64 { dd + 360f64 } else { dd })
                }
            }
            Angle::Radians(r) => {
                let r = *r;

                if 0f64 <= r && r <= 2f64 * PI {
                    Angle::Radians(r)
                } else {
                    let rr = r % (2f64 * PI);
                    Angle::Radians(if rr < 0f64 { rr + 2f64 * PI } else { rr })
                }
            }
        }
    }

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

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use super::Angle;

    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn angle_normalize_works() {
        assert_eq!(Angle::Degrees(360f64).to_degrees(), 360f64);
        assert_eq!(Angle::Radians(2f64 * PI).to_radians(), 2f64 * PI);

        assert_approx_eq!(
            Angle::Degrees(361.2f64).normalized().to_degrees(),
            1.2f64,
            1e-12f64
        );

        assert_approx_eq!(
            Angle::Degrees(-270f64).normalized().to_degrees(),
            90f64,
            1e-12f64
        );

        assert_approx_eq!(
            Angle::Radians(2.5f64 * PI).normalized().to_radians(),
            0.5f64 * PI,
            1e-12f64
        );

        assert_approx_eq!(
            Angle::Radians(-0.5f64 * PI).normalized().to_radians(),
            1.5f64 * PI,
            1e-12f64
        );
    }
}
