use std::marker::PhantomData;

use image::{GenericImage, GenericImageView, Rgba, RgbaImage};

use crate::{
    basic::BasicRenderer,
    types::{Angle, Circle, Rect},
    Renderer,
};

pub struct AntiAliasingRender<I> {
    _phantom_data: PhantomData<I>,
}

impl<I> Default for AntiAliasingRender<I> {
    fn default() -> Self {
        Self {
            _phantom_data: PhantomData::default(),
        }
    }
}

impl Renderer for AntiAliasingRender<RgbaImage> {
    type Image = RgbaImage;
    type Pixel = <RgbaImage as GenericImageView>::Pixel;

    fn draw_line(
        &self,
        img: &mut Self::Image,
        from: (u32, u32),
        to: (u32, u32),
        color: Self::Pixel,
    ) {
        let (steep, x0, x1, y0, y1) = {
            let (mut x0, mut y0) = (from.0 as i64, from.1 as i64);
            let (mut x1, mut y1) = (to.0 as i64, to.1 as i64);

            let steep = (y1 - y0).abs() > (x1 - x0).abs();

            if steep {
                std::mem::swap(&mut x0, &mut y0);
                std::mem::swap(&mut x1, &mut y1);
            }

            if x0 > x1 {
                std::mem::swap(&mut x0, &mut x1);
                std::mem::swap(&mut y0, &mut y1);
            }

            (steep, x0, x1, y0, y1)
        };

        let dx = x1 - x0;
        let gradient = if dx == 0 {
            1f64
        } else {
            (y1 - y0) as f64 / dx as f64
        };

        let mut intersect_y = y0 as f64;

        if steep {
            for x in x0..=x1 {
                img.blend_pixel(
                    intersect_y as u32,
                    x as u32,
                    rgba_u8_pixel_with_brightness(color, 1f64 - intersect_y.fract()),
                );
                img.blend_pixel(
                    intersect_y as u32 + 1,
                    x as u32,
                    rgba_u8_pixel_with_brightness(color, intersect_y.fract()),
                );
                intersect_y += gradient;
            }
        } else {
            for x in x0..=x1 {
                img.blend_pixel(
                    x as u32,
                    intersect_y as u32,
                    rgba_u8_pixel_with_brightness(color, 1f64 - intersect_y.fract()),
                );
                img.blend_pixel(
                    x as u32,
                    intersect_y as u32 + 1,
                    rgba_u8_pixel_with_brightness(color, intersect_y.fract()),
                );
                intersect_y += gradient;
            }
        }
    }

    fn draw_rect(&self, img: &mut Self::Image, rect: Rect, color: Self::Pixel) {
        BasicRenderer::default().draw_rect(img, rect, color);
    }

    fn draw_filled_rect(&self, img: &mut Self::Image, rect: Rect, color: Self::Pixel) {
        BasicRenderer::default().draw_filled_rect(img, rect, color);
    }

    fn draw_circle(&self, img: &mut Self::Image, circle: Circle, color: Self::Pixel) {
        let radius = circle.radius();
        if radius == 0 {
            return;
        }

        let center = circle.center();
        let (x0, y0) = center;

        img.blend_pixel(x0 + radius, y0, color);
        img.blend_pixel(x0 - radius, y0, color);
        img.blend_pixel(x0, y0 + radius, color);
        img.blend_pixel(x0, y0 - radius, color);

        let mut x = radius;
        let mut y = 0;

        while x > y {
            y += 1;

            let real_x = ((radius * radius - y * y) as f64).sqrt();
            let real_x_ceil = real_x.ceil();
            x = real_x_ceil as u32;
            let k = real_x_ceil - real_x;

            let c1 = rgba_u8_pixel_with_brightness(color, 1f64 - k);
            let c2 = rgba_u8_pixel_with_brightness(color, k);

            img.blend_pixel(x0 + x, y0 + y, c1);
            img.blend_pixel(x0 + x - 1, y0 + y, c2);

            img.blend_pixel(x0 + y, y0 + x, c1);
            img.blend_pixel(x0 + y, y0 + x - 1, c2);

            img.blend_pixel(x0 - y, y0 + x, c1);
            img.blend_pixel(x0 - y, y0 + x - 1, c2);

            img.blend_pixel(x0 - x, y0 + y, c1);
            img.blend_pixel(x0 - x + 1, y0 + y, c2);

            img.blend_pixel(x0 - x, y0 - y, c1);
            img.blend_pixel(x0 - x + 1, y0 - y, c2);

            img.blend_pixel(x0 - y, y0 - x, c1);
            img.blend_pixel(x0 - y, y0 - x + 1, c2);

            img.blend_pixel(x0 + y, y0 - x, c1);
            img.blend_pixel(x0 + y, y0 - x + 1, c2);

            img.blend_pixel(x0 + x, y0 - y, c1);
            img.blend_pixel(x0 + x - 1, y0 - y, c2);
        }
    }

    fn draw_filled_circle(&self, img: &mut Self::Image, circle: Circle, color: Self::Pixel) {
        let radius = circle.radius();
        if radius == 0 {
            return;
        }

        let center = circle.center();
        let (x0, y0) = center;

        img.blend_pixel(x0 + radius, y0, color);
        img.blend_pixel(x0 - radius, y0, color);
        img.blend_pixel(x0, y0 + radius, color);
        img.blend_pixel(x0, y0 - radius, color);

        self.draw_line(img, (x0 - radius, y0), (x0 + radius, y0), color);
        self.draw_line(img, (x0, y0 - radius), (x0, y0 + radius), color);

        let mut x = radius;
        let mut y = 0;

        while x > y {
            y += 1;

            let real_x = ((radius * radius - y * y) as f64).sqrt();
            let real_x_ceil = real_x.ceil();
            x = real_x_ceil as u32;
            let k = real_x_ceil - real_x;

            let c1 = rgba_u8_pixel_with_brightness(color, 1f64 - k);
            let c2 = rgba_u8_pixel_with_brightness(color, k);

            img.blend_pixel(x0 + x, y0 + y, c1);
            img.blend_pixel(x0 + x - 1, y0 + y, c2);

            img.blend_pixel(x0 + y, y0 + x, c1);
            img.blend_pixel(x0 + y, y0 + x - 1, c2);

            img.blend_pixel(x0 - y, y0 + x, c1);
            img.blend_pixel(x0 - y, y0 + x - 1, c2);

            img.blend_pixel(x0 - x, y0 + y, c1);
            img.blend_pixel(x0 - x + 1, y0 + y, c2);

            img.blend_pixel(x0 - x, y0 - y, c1);
            img.blend_pixel(x0 - x + 1, y0 - y, c2);

            img.blend_pixel(x0 - y, y0 - x, c1);
            img.blend_pixel(x0 - y, y0 - x + 1, c2);

            img.blend_pixel(x0 + y, y0 - x, c1);
            img.blend_pixel(x0 + y, y0 - x + 1, c2);

            img.blend_pixel(x0 + x, y0 - y, c1);
            img.blend_pixel(x0 + x - 1, y0 - y, c2);

            if x0 >= x {
                self.draw_line(
                    img,
                    ((x0 - x + 1), (y0 + y)),
                    ((x0 + x - 1), (y0 + y)),
                    color,
                );
            }

            if x0 >= y {
                self.draw_line(
                    img,
                    ((x0 - y), (y0 + x - 1)),
                    ((x0 + y), (y0 + x - 1)),
                    color,
                );
            }

            if x0 >= x && y0 >= y {
                self.draw_line(
                    img,
                    ((x0 - x + 1), (y0 - y)),
                    ((x0 + x - 1), (y0 - y)),
                    color,
                );
            }

            if x0 >= y && y0 >= x {
                self.draw_line(
                    img,
                    ((x0 - y), (y0 - x + 1)),
                    ((x0 + y), (y0 - x + 1)),
                    color,
                );
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

        let mut x = radius;
        let mut y = 0;

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

        let a1 = 0f64;
        if a1 >= start.to_degrees() && a1 <= end.to_degrees() {
            img.blend_pixel(x0 + radius, y0, color);
        }

        let a1 = 90f64;
        if a1 >= start.to_degrees() && a1 <= end.to_degrees() {
            img.blend_pixel(x0, y0 + radius, color);
        }

        let a1 = 180f64;
        if a1 >= start.to_degrees() && a1 <= end.to_degrees() {
            img.blend_pixel(x0 - radius, y0, color);
        }

        let a1 = 270f64;
        if a1 >= start.to_degrees() && a1 <= end.to_degrees() {
            img.blend_pixel(x0, y0 - radius, color);
        }

        while x > y {
            y += 1;

            let real_x = ((radius * radius - y * y) as f64).sqrt();
            let real_x_ceil = real_x.ceil();
            x = real_x_ceil as u32;
            let k = real_x_ceil - real_x;

            let c1 = rgba_u8_pixel_with_brightness(color, 1f64 - k);
            let c2 = rgba_u8_pixel_with_brightness(color, k);

            let a1 = (y as f64).atan2(x as f64).to_degrees();
            let a1 = if a1 < 0f64 { a1 + 360f64 } else { a1 };
            if a1 >= start.to_degrees() && a1 <= end.to_degrees() {
                img.blend_pixel(x0 + x, y0 + y, c1);
                img.blend_pixel(x0 + x - 1, y0 + y, c2);
            }

            let a1 = (x as f64).atan2(y as f64).to_degrees();
            let a1 = if a1 < 0f64 { a1 + 360f64 } else { a1 };
            if a1 >= start.to_degrees() && a1 <= end.to_degrees() {
                img.blend_pixel(x0 + y, y0 + x, c1);
                img.blend_pixel(x0 + y, y0 + x - 1, c2);
            }

            let a1 = (x as f64).atan2(-(y as f64)).to_degrees();
            let a1 = if a1 < 0f64 { a1 + 360f64 } else { a1 };
            if a1 >= start.to_degrees() && a1 <= end.to_degrees() {
                img.blend_pixel(x0 - y, y0 + x, c1);
                img.blend_pixel(x0 - y, y0 + x - 1, c2);
            }

            let a1 = (y as f64).atan2(-(x as f64)).to_degrees();
            let a1 = if a1 < 0f64 { a1 + 360f64 } else { a1 };
            if a1 >= start.to_degrees() && a1 <= end.to_degrees() {
                img.blend_pixel(x0 - x, y0 + y, c1);
                img.blend_pixel(x0 - x + 1, y0 + y, c2);
            }

            let a1 = (-(y as f64)).atan2(-(x as f64)).to_degrees();
            let a1 = if a1 < 0f64 { a1 + 360f64 } else { a1 };
            if a1 >= start.to_degrees() && a1 <= end.to_degrees() {
                img.blend_pixel(x0 - x, y0 - y, c1);
                img.blend_pixel(x0 - x + 1, y0 - y, c2);
            }

            let a1 = (-(x as f64)).atan2(-(y as f64)).to_degrees();
            let a1 = if a1 < 0f64 { a1 + 360f64 } else { a1 };
            if a1 >= start.to_degrees() && a1 <= end.to_degrees() {
                img.blend_pixel(x0 - y, y0 - x, c1);
                img.blend_pixel(x0 - y, y0 - x + 1, c2);
            }

            let a1 = (-(x as f64)).atan2(y as f64).to_degrees();
            let a1 = if a1 < 0f64 { a1 + 360f64 } else { a1 };
            if a1 >= start.to_degrees() && a1 <= end.to_degrees() {
                img.blend_pixel(x0 + y, y0 - x, c1);
                img.blend_pixel(x0 + y, y0 - x + 1, c2);
            }

            let a1 = (-(y as f64)).atan2(x as f64).to_degrees();
            let a1 = if a1 < 0f64 { a1 + 360f64 } else { a1 };
            if a1 >= start.to_degrees() && a1 <= end.to_degrees() {
                img.blend_pixel(x0 + x, y0 - y, c1);
                img.blend_pixel(x0 + x - 1, y0 - y, c2);
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

        let mut x = radius;
        let mut y = 0;

        while start.to_degrees() < 0f64 {
            start = Angle::Degrees(start.to_degrees() + 360f64);
        }
        while start.to_degrees() > 360f64 {
            start = Angle::Degrees(start.to_degrees() - 360f64);
        }
        while end.to_degrees() < 0f64 {
            end = Angle::Degrees(end.to_degrees() + 360f64);
        }
        while end.to_degrees() > 360f64 {
            end = Angle::Degrees(end.to_degrees() - 360f64);
        }

        let (start, end) = if start.to_degrees() > end.to_degrees() {
            (end, start)
        } else {
            (start, end)
        };

        let filter_fn = |x, y| {
            let a1 = (y as f64 - y0 as f64)
                .atan2(x as f64 - x0 as f64)
                .to_degrees();
            let a1 = if a1 < 0f64 { a1 + 360f64 } else { a1 };

            a1 >= start.to_degrees() && a1 <= end.to_degrees()
        };

        let basic_renderer = BasicRenderer::default();

        img.blend_pixel(x0, y0, color);

        basic_renderer.filtered_draw_line(
            img,
            (x0 - radius, y0),
            (x0 + radius, y0),
            color,
            filter_fn,
        );
        basic_renderer.filtered_draw_line(
            img,
            (x0, y0 - radius),
            (x0, y0 + radius),
            color,
            filter_fn,
        );

        while x > y {
            y += 1;

            let real_x = ((radius * radius - y * y) as f64).sqrt();
            let real_x_ceil = real_x.ceil();
            x = real_x_ceil as u32;
            let k = real_x_ceil - real_x;

            let c1 = rgba_u8_pixel_with_brightness(color, 1f64 - k);
            let c2 = rgba_u8_pixel_with_brightness(color, k);

            let a1 = (y as f64).atan2(x as f64).to_degrees();
            let a1 = if a1 < 0f64 { a1 + 360f64 } else { a1 };
            if a1 >= start.to_degrees() && a1 <= end.to_degrees() {
                img.blend_pixel(x0 + x, y0 + y, c1);
                img.blend_pixel(x0 + x - 1, y0 + y, c2);
            }

            let a1 = (x as f64).atan2(y as f64).to_degrees();
            let a1 = if a1 < 0f64 { a1 + 360f64 } else { a1 };
            if a1 >= start.to_degrees() && a1 <= end.to_degrees() {
                img.blend_pixel(x0 + y, y0 + x, c1);
                img.blend_pixel(x0 + y, y0 + x - 1, c2);
            }

            let a1 = (x as f64).atan2(-(y as f64)).to_degrees();
            let a1 = if a1 < 0f64 { a1 + 360f64 } else { a1 };
            if a1 >= start.to_degrees() && a1 <= end.to_degrees() {
                img.blend_pixel(x0 - y, y0 + x, c1);
                img.blend_pixel(x0 - y, y0 + x - 1, c2);
            }

            let a1 = (y as f64).atan2(-(x as f64)).to_degrees();
            let a1 = if a1 < 0f64 { a1 + 360f64 } else { a1 };
            if a1 >= start.to_degrees() && a1 <= end.to_degrees() {
                img.blend_pixel(x0 - x, y0 + y, c1);
                img.blend_pixel(x0 - x + 1, y0 + y, c2);
            }

            let a1 = (-(y as f64)).atan2(-(x as f64)).to_degrees();
            let a1 = if a1 < 0f64 { a1 + 360f64 } else { a1 };
            if a1 >= start.to_degrees() && a1 <= end.to_degrees() {
                img.blend_pixel(x0 - x, y0 - y, c1);
                img.blend_pixel(x0 - x + 1, y0 - y, c2);
            }

            let a1 = (-(x as f64)).atan2(-(y as f64)).to_degrees();
            let a1 = if a1 < 0f64 { a1 + 360f64 } else { a1 };
            if a1 >= start.to_degrees() && a1 <= end.to_degrees() {
                img.blend_pixel(x0 - y, y0 - x, c1);
                img.blend_pixel(x0 - y, y0 - x + 1, c2);
            }

            let a1 = (-(x as f64)).atan2(y as f64).to_degrees();
            let a1 = if a1 < 0f64 { a1 + 360f64 } else { a1 };
            if a1 >= start.to_degrees() && a1 <= end.to_degrees() {
                img.blend_pixel(x0 + y, y0 - x, c1);
                img.blend_pixel(x0 + y, y0 - x + 1, c2);
            }

            let a1 = (-(y as f64)).atan2(x as f64).to_degrees();
            let a1 = if a1 < 0f64 { a1 + 360f64 } else { a1 };
            if a1 >= start.to_degrees() && a1 <= end.to_degrees() {
                img.blend_pixel(x0 + x, y0 - y, c1);
                img.blend_pixel(x0 + x - 1, y0 - y, c2);
            }

            if x0 >= x {
                basic_renderer.filtered_draw_line(
                    img,
                    (x0 - x + 1, y0 + y),
                    (x0 + x - 1, y0 + y),
                    color,
                    filter_fn,
                );
            }

            if x0 >= y {
                basic_renderer.filtered_draw_line(
                    img,
                    ((x0 - y), (y0 + x - 1)),
                    ((x0 + y), (y0 + x - 1)),
                    color,
                    filter_fn,
                );
            }

            if x0 >= x && y0 >= y {
                basic_renderer.filtered_draw_line(
                    img,
                    ((x0 - x + 1), (y0 - y)),
                    ((x0 + x - 1), (y0 - y)),
                    color,
                    filter_fn,
                );
            }

            if x0 >= y && y0 >= x {
                basic_renderer.filtered_draw_line(
                    img,
                    ((x0 - y), (y0 - x + 1)),
                    ((x0 + y), (y0 - x + 1)),
                    color,
                    filter_fn,
                );
            }
        }
    }
}

fn rgba_u8_pixel_with_brightness(pixel: Rgba<u8>, brightness: f64) -> Rgba<u8> {
    Rgba([
        pixel.0[0],
        pixel.0[1],
        pixel.0[2],
        (255.0 * brightness) as u8,
    ])
}
