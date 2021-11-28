use std::marker::PhantomData;

use image::{GenericImage, GenericImageView};

use crate::{
    types::{Angle, Circle, Rect},
    Renderer,
};

pub struct BasicRenderer<I> {
    _phantom_data: PhantomData<I>,
}

impl<I> Default for BasicRenderer<I> {
    fn default() -> Self {
        Self {
            _phantom_data: PhantomData::default(),
        }
    }
}

impl<I: GenericImage> Renderer for BasicRenderer<I> {
    type Image = I;
    type Pixel = <I as GenericImageView>::Pixel;

    fn draw_line(
        &self,
        img: &mut Self::Image,
        from: (u32, u32),
        to: (u32, u32),
        color: Self::Pixel,
    ) {
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
            img.blend_pixel(x as u32, y as u32, color);

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

    fn draw_rect(&self, img: &mut Self::Image, rect: Rect, color: Self::Pixel) {
        for x in rect.left()..=rect.right() {
            img.blend_pixel(x, rect.top(), color);
            img.blend_pixel(x, rect.bottom(), color);
        }

        for y in rect.top()..=rect.bottom() {
            img.blend_pixel(rect.left(), y, color);
            img.blend_pixel(rect.right(), y, color);
        }
    }

    fn draw_filled_rect(&self, img: &mut Self::Image, rect: Rect, color: Self::Pixel) {
        for x in rect.left()..=rect.right() {
            for y in rect.top()..=rect.bottom() {
                img.blend_pixel(x, y, color);
            }
        }
    }

    fn draw_circle(&self, img: &mut Self::Image, circle: Circle, color: Self::Pixel) {
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
            img.blend_pixel(x0 + x, y0 + y, color);
            img.blend_pixel(x0 + y, y0 + x, color);
            img.blend_pixel(x0 - y, y0 + x, color);
            img.blend_pixel(x0 - x, y0 + y, color);
            img.blend_pixel(x0 - x, y0 - y, color);
            img.blend_pixel(x0 - y, y0 - x, color);
            img.blend_pixel(x0 + y, y0 - x, color);
            img.blend_pixel(x0 + x, y0 - y, color);

            x += 1;
            if p < 0 {
                p += 2 * x as i64 + 1;
            } else {
                y -= 1;
                p += 2 * (x as i64 - y as i64) + 1;
            }
        }
    }

    fn draw_filled_circle(&self, img: &mut Self::Image, circle: Circle, color: Self::Pixel) {
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

    fn draw_arc(
        &self,
        img: &mut Self::Image,
        circle: Circle,
        mut start: Angle,
        mut end: Angle,
        color: Self::Pixel,
    ) {
        let radius = circle.radius();
        if radius == 0 {
            return;
        }

        let center = circle.center();
        let (x0, y0) = center;

        let mut x = 0;
        let mut y = radius;
        let mut p = 1 - radius as i64;

        while start.to_degrees() < 0f64 {
            start = Angle::Degrees(start.to_degrees() + 360f64);
        }
        while start.to_degrees() >= 360f64 {
            start = Angle::Degrees(start.to_degrees() - 360f64);
        }
        while end.to_degrees() < 0f64 {
            end = Angle::Degrees(end.to_degrees() + 360f64);
        }
        while end.to_degrees() >= 360f64 {
            end = Angle::Degrees(end.to_degrees() - 360f64);
        }

        let (start, end) = if start.to_degrees() > end.to_degrees() {
            (end, start)
        } else {
            (start, end)
        };

        while x <= y {
            let a1 = (y as f64).atan2(x as f64).to_degrees();
            let a1 = if a1 < 0f64 { a1 + 360f64 } else { a1 };
            if a1 >= start.to_degrees() && a1 <= end.to_degrees() {
                img.blend_pixel(x0 + x, y0 + y, color);
            }

            let a2 = (x as f64).atan2(y as f64).to_degrees();
            let a2 = if a2 < 0f64 { a2 + 360f64 } else { a2 };
            if a2 >= start.to_degrees() && a2 <= end.to_degrees() {
                img.blend_pixel(x0 + y, y0 + x, color);
            }

            let a3 = (x as f64).atan2(-(y as f64)).to_degrees();
            let a3 = if a3 < 0f64 { a3 + 360f64 } else { a3 };
            if a3 >= start.to_degrees() && a3 <= end.to_degrees() {
                img.blend_pixel(x0 - y, y0 + x, color);
            }

            let a4 = (y as f64).atan2(-(x as f64)).to_degrees();
            let a4 = if a4 < 0f64 { a4 + 360f64 } else { a4 };
            if a4 >= start.to_degrees() && a4 <= end.to_degrees() {
                img.blend_pixel(x0 - x, y0 + y, color);
            }

            let a5 = (-(y as f64)).atan2(-(x as f64)).to_degrees();
            let a5 = if a5 < 0f64 { a5 + 360f64 } else { a5 };
            if a5 >= start.to_degrees() && a5 <= end.to_degrees() {
                img.blend_pixel(x0 - x, y0 - y, color);
            }

            let a6 = (-(x as f64)).atan2(-(y as f64)).to_degrees();
            let a6 = if a6 < 0f64 { a6 + 360f64 } else { a6 };
            if a6 >= start.to_degrees() && a6 <= end.to_degrees() {
                img.blend_pixel(x0 - y, y0 - x, color);
            }

            let a7 = (-(x as f64)).atan2(y as f64).to_degrees();
            let a7 = if a7 < 0f64 { a7 + 360f64 } else { a7 };
            if a7 >= start.to_degrees() && a7 <= end.to_degrees() {
                img.blend_pixel(x0 + y, y0 - x, color);
            }

            let a8 = (-(y as f64)).atan2(x as f64).to_degrees();
            let a8 = if a8 < 0f64 { a8 + 360f64 } else { a8 };
            if a8 >= start.to_degrees() && a8 <= end.to_degrees() {
                img.blend_pixel(x0 + x, y0 - y, color);
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
