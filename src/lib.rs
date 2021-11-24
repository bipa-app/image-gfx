use image::{GenericImage, GenericImageView};

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

pub struct Renderer;

impl Default for Renderer {
    fn default() -> Self {
        Self
    }
}

impl Renderer {
    pub fn draw_line<I>(
        &self,
        img: &mut I,
        from: (u32, u32),
        to: (u32, u32),
        color: <I as GenericImageView>::Pixel,
    ) where
        I: GenericImage,
    {
        let (x0, y0) = from;
        let (x1, y1) = to;
        let (dx, dy) = (
            (x1 as i64 - x0 as i64).abs(),
            -((y1 as i64 - y0 as i64).abs()),
        );
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut err = dx + dy;
        let (mut x, mut y) = (x0 as i64, y0 as i64);

        loop {
            img.put_pixel(x as u32, y as u32, color);

            if x as u32 == x1 && y as u32 == y1 {
                break;
            }

            let err2 = err * 2;

            if err2 >= dy {
                err += dy;
                x += sx;
            }

            if err2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn draw_rect<I>(&self, img: &mut I, rect: Rect, color: <I as GenericImageView>::Pixel)
    where
        I: GenericImage,
    {
        for x in rect.left()..=rect.right() {
            img.put_pixel(x, rect.top(), color);
            img.put_pixel(x, rect.bottom(), color);
        }

        for y in rect.top()..=rect.bottom() {
            img.put_pixel(rect.left(), y, color);
            img.put_pixel(rect.right(), y, color);
        }
    }

    pub fn draw_filled_rect<I>(
        &self,
        img: &mut I,
        rect: Rect,
        color: <I as GenericImageView>::Pixel,
    ) where
        I: GenericImage,
    {
        for x in rect.left()..=rect.right() {
            for y in rect.top()..=rect.bottom() {
                img.put_pixel(x, y, color);
            }
        }
    }

    pub fn draw_circle<I>(&self, img: &mut I, circle: Circle, color: <I as GenericImageView>::Pixel)
    where
        I: GenericImage,
    {
        let radius = circle.radius();
        if radius == 0 {
            return;
        }

        let center = circle.center();
        let (x0, y0) = center;

        let mut x = 0;
        let mut y = radius;
        let mut p = 1 - radius as i64;

        while x <= y {
            img.put_pixel(x0 + x, y0 + y, color);
            img.put_pixel(x0 + y, y0 + x, color);
            img.put_pixel(x0 - y, y0 + x, color);
            img.put_pixel(x0 - x, y0 + y, color);
            img.put_pixel(x0 - x, y0 - y, color);
            img.put_pixel(x0 - y, y0 - x, color);
            img.put_pixel(x0 + y, y0 - x, color);
            img.put_pixel(x0 + x, y0 - y, color);

            x += 1;
            if p < 0 {
                p += 2 * x as i64 + 1;
            } else {
                y -= 1;
                p += 2 * (x as i64 - y as i64) + 1;
            }
        }
    }

    pub fn draw_filled_circle<I>(
        &self,
        img: &mut I,
        circle: Circle,
        color: <I as GenericImageView>::Pixel,
    ) where
        I: GenericImage,
    {
        let r = circle.radius();
        if r == 0 {
            return;
        }

        let center = circle.center();
        let (x0, y0) = center;

        self.draw_line(img, (x0 + r, y0), (x0 - r, y0), color);
        self.draw_line(img, (x0, y0 + r), (x0, y0 - r), color);

        let mut p: i64 = 1 - r as i64;
        let mut x = 0;
        let mut y = r;

        while x <= y {
            if x0 >= x {
                self.draw_line(img, ((x0 - x), (y0 + y)), ((x0 + x), (y0 + y)), color);
            }

            if x0 >= y {
                self.draw_line(img, ((x0 - y), (y0 + x)), ((x0 + y), (y0 + x)), color);
            }

            if x0 >= x && y0 >= y {
                self.draw_line(img, ((x0 - x), (y0 - y)), ((x0 + x), (y0 - y)), color);
            }

            if x0 >= y && y0 >= x {
                self.draw_line(img, ((x0 - y), (y0 - x)), ((x0 + y), (y0 - x)), color);
            }

            x += 1;
            if p < 0 {
                p += 2 * x as i64 + 1;
            } else {
                y -= 1;
                p += 2 * (x as i64 - y as i64) + 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut img = image::RgbaImage::from_pixel(100, 100, image::Rgba([255, 255, 255, 255]));

        let r = Renderer::default();

        r.draw_line(&mut img, (90, 70), (90, 80), image::Rgba([0, 0, 255, 255]));
        r.draw_line(&mut img, (90, 70), (80, 60), image::Rgba([0, 0, 255, 255]));

        r.draw_rect(
            &mut img,
            Rect::new(8, 8, 16, 16),
            image::Rgba([255, 0, 0, 255]),
        );

        r.draw_filled_rect(
            &mut img,
            Rect::new(16, 16, 8, 8),
            image::Rgba([255, 0, 0, 255]),
        );

        r.draw_circle(
            &mut img,
            Circle::new((50, 50), 20),
            image::Rgba([0, 255, 0, 255]),
        );

        r.draw_filled_circle(
            &mut img,
            Circle::new((50, 50), 10),
            image::Rgba([0, 255, 0, 255]),
        );

        img.save("./test.png").unwrap();
    }
}
