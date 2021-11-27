use std::marker::PhantomData;

use image::{GenericImage, GenericImageView};

use crate::{
    types::{Circle, Rect},
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
}
