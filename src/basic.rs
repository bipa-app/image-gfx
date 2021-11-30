use std::marker::PhantomData;

use image::{GenericImage, GenericImageView};

use crate::{
    geom::inside_arc,
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

impl<I: GenericImage> BasicRenderer<I> {
    pub(crate) fn filtered_draw_line<F: Fn(u32, u32) -> bool>(
        &self,
        img: &mut I,
        from: (u32, u32),
        to: (u32, u32),
        color: <I as GenericImageView>::Pixel,
        filter: F,
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
            if filter(x as u32, y as u32) {
                img.blend_pixel(x as u32, y as u32, color);
            }

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
        self.filtered_draw_line(img, from, to, color, |_, _| true);
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

        start.normalize();
        end.normalize();

        let (start, end) = if start.to_degrees() > end.to_degrees() {
            (end, start)
        } else {
            (start, end)
        };

        while x <= y {
            if inside_arc((x as f64, y as f64), start, end) {
                img.blend_pixel(x0 + x, y0 + y, color);
            }

            if inside_arc((y as f64, x as f64), start, end) {
                img.blend_pixel(x0 + y, y0 + x, color);
            }

            if inside_arc((-(y as f64), x as f64), start, end) {
                img.blend_pixel(x0 - y, y0 + x, color);
            }

            if inside_arc((-(x as f64), y as f64), start, end) {
                img.blend_pixel(x0 - x, y0 + y, color);
            }

            if inside_arc((-(x as f64), -(y as f64)), start, end) {
                img.blend_pixel(x0 - x, y0 - y, color);
            }

            if inside_arc((-(y as f64), -(x as f64)), start, end) {
                img.blend_pixel(x0 - y, y0 - x, color);
            }

            if inside_arc((y as f64, -(x as f64)), start, end) {
                img.blend_pixel(x0 + y, y0 - x, color);
            }

            if inside_arc((x as f64, -(y as f64)), start, end) {
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

    fn draw_filled_arc(
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

        start.normalize();
        end.normalize();

        let (start, end) = if start.to_degrees() > end.to_degrees() {
            (end, start)
        } else {
            (start, end)
        };

        let filter_fn = |x, y| inside_arc((x as f64 - x0 as f64, y as f64 - y0 as f64), start, end);

        img.blend_pixel(x0, y0, color);

        self.filtered_draw_line(img, (x0 - radius, y0), (x0 + radius, y0), color, filter_fn);
        self.filtered_draw_line(img, (x0, y0 - radius), (x0, y0 + radius), color, filter_fn);

        while x <= y {
            if x0 >= x {
                self.filtered_draw_line(img, (x0 - x, y0 + y), (x0 + x, y0 + y), color, filter_fn);
            }

            if x0 >= y {
                self.filtered_draw_line(
                    img,
                    ((x0 - y), (y0 + x)),
                    ((x0 + y), (y0 + x)),
                    color,
                    filter_fn,
                );
            }

            if x0 >= x && y0 >= y {
                self.filtered_draw_line(
                    img,
                    ((x0 - x), (y0 - y)),
                    ((x0 + x), (y0 - y)),
                    color,
                    filter_fn,
                );
            }

            if x0 >= y && y0 >= x {
                self.filtered_draw_line(
                    img,
                    ((x0 - y), (y0 - x)),
                    ((x0 + y), (y0 - x)),
                    color,
                    filter_fn,
                );
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

    fn draw_rounded_rect(
        &self,
        img: &mut Self::Image,
        rect: Rect,
        corner_radius: u32,
        color: Self::Pixel,
    ) {
        self.draw_line(
            img,
            (rect.left(), rect.top() + corner_radius),
            (rect.left(), rect.bottom() - corner_radius),
            color,
        );
        self.draw_arc(
            img,
            Circle::new(
                (rect.left() + corner_radius, rect.top() + corner_radius),
                corner_radius,
            ),
            Angle::Degrees(180f64),
            Angle::Degrees(270f64),
            color,
        );
        self.draw_arc(
            img,
            Circle::new(
                (rect.left() + corner_radius, rect.bottom() - corner_radius),
                corner_radius,
            ),
            Angle::Degrees(90f64),
            Angle::Degrees(180f64),
            color,
        );

        self.draw_line(
            img,
            (rect.right(), rect.top() + corner_radius),
            (rect.right(), rect.bottom() - corner_radius),
            color,
        );
        self.draw_arc(
            img,
            Circle::new(
                (rect.right() - corner_radius, rect.top() + corner_radius),
                corner_radius,
            ),
            Angle::Degrees(270f64),
            Angle::Degrees(360f64),
            color,
        );
        self.draw_arc(
            img,
            Circle::new(
                (rect.right() - corner_radius, rect.bottom() - corner_radius),
                corner_radius,
            ),
            Angle::Degrees(0f64),
            Angle::Degrees(90f64),
            color,
        );

        self.draw_line(
            img,
            (rect.left() + corner_radius, rect.top()),
            (rect.right() - corner_radius, rect.top()),
            color,
        );
        self.draw_line(
            img,
            (rect.left() + corner_radius, rect.bottom()),
            (rect.right() - corner_radius, rect.bottom()),
            color,
        );
    }

    fn draw_filled_rounded_rect(
        &self,
        img: &mut Self::Image,
        rect: Rect,
        corner_radius: u32,
        color: Self::Pixel,
    ) {
        self.draw_filled_rect(
            img,
            Rect::new(
                rect.left() + corner_radius,
                rect.top(),
                rect.width() - corner_radius * 2,
                rect.height(),
            ),
            color,
        );

        self.draw_filled_rect(
            img,
            Rect::new(
                rect.left(),
                rect.top() + corner_radius,
                corner_radius,
                rect.height() - corner_radius * 2,
            ),
            color,
        );

        self.draw_filled_rect(
            img,
            Rect::new(
                rect.right() - corner_radius,
                rect.top() + corner_radius,
                corner_radius,
                rect.height() - corner_radius * 2,
            ),
            color,
        );

        self.draw_filled_arc(
            img,
            Circle::new(
                (rect.left() + corner_radius, rect.top() + corner_radius),
                corner_radius,
            ),
            Angle::Degrees(180f64),
            Angle::Degrees(270f64),
            color,
        );

        self.draw_filled_arc(
            img,
            Circle::new(
                (rect.left() + corner_radius, rect.bottom() - corner_radius),
                corner_radius,
            ),
            Angle::Degrees(90f64),
            Angle::Degrees(180f64),
            color,
        );

        self.draw_filled_arc(
            img,
            Circle::new(
                (rect.right() - corner_radius, rect.top() + corner_radius),
                corner_radius,
            ),
            Angle::Degrees(270f64),
            Angle::Degrees(360f64),
            color,
        );

        self.draw_filled_arc(
            img,
            Circle::new(
                (rect.right() - corner_radius, rect.bottom() - corner_radius),
                corner_radius,
            ),
            Angle::Degrees(0f64),
            Angle::Degrees(90f64),
            color,
        );
    }
}
