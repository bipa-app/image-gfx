pub mod antialiased;
pub mod basic;
pub mod geom;
pub mod types;

use image::{GenericImage, GenericImageView};
use types::*;

pub trait Renderer {
    type Image;
    type Pixel;

    fn draw_line(
        &self,
        img: &mut Self::Image,
        from: (u32, u32),
        to: (u32, u32),
        color: Self::Pixel,
    );

    fn draw_rect(&self, img: &mut Self::Image, rect: Rect, color: Self::Pixel);
    fn draw_filled_rect(&self, img: &mut Self::Image, rect: Rect, color: Self::Pixel);

    fn draw_circle(&self, img: &mut Self::Image, circle: Circle, color: Self::Pixel);
    fn draw_filled_circle(&self, img: &mut Self::Image, circle: Circle, color: Self::Pixel);

    fn draw_arc(
        &self,
        img: &mut Self::Image,
        circle: Circle,
        start: Angle,
        end: Angle,
        color: Self::Pixel,
    );
    fn draw_filled_arc(
        &self,
        img: &mut Self::Image,
        circle: Circle,
        start: Angle,
        end: Angle,
        color: Self::Pixel,
    );

    fn draw_rounded_rect(
        &self,
        img: &mut Self::Image,
        rect: Rect,
        corner_radius: u32,
        color: Self::Pixel,
    );
    fn draw_filled_rounded_rect(
        &self,
        img: &mut Self::Image,
        rect: Rect,
        corner_radius: u32,
        color: Self::Pixel,
    );
}

pub(crate) fn blend_pixel<I: GenericImage>(
    img: &mut I,
    x: u32,
    y: u32,
    color: <I as GenericImageView>::Pixel,
) {
    if img.in_bounds(x, y) {
        img.blend_pixel(x, y, color);
    }
}

#[cfg(test)]
mod tests {
    use crate::{antialiased::AntiAliasingRender, basic::BasicRenderer};

    use super::*;

    #[test]
    fn it_works() {
        let mut img = image::RgbaImage::from_pixel(100, 100, image::Rgba([255, 255, 255, 255]));

        let r = BasicRenderer::default();
        let aar = AntiAliasingRender::default();

        r.draw_line(&mut img, (90, 70), (90, 80), image::Rgba([0, 0, 255, 255]));
        r.draw_line(&mut img, (90, 70), (80, 60), image::Rgba([0, 0, 255, 255]));

        aar.draw_line(&mut img, (97, 70), (80, 60), image::Rgba([255, 0, 0, 255]));

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

        aar.draw_filled_circle(
            &mut img,
            Circle::new((50, 50), 30),
            image::Rgba([0, 0, 255, 255]),
        );
        r.draw_circle(
            &mut img,
            Circle::new((50, 50), 30),
            image::Rgba([255, 0, 0, 255]),
        );

        r.draw_circle(
            &mut img,
            Circle::new((50, 50), 20),
            image::Rgba([0, 255, 0, 255]),
        );

        aar.draw_circle(
            &mut img,
            Circle::new((50, 50), 15),
            image::Rgba([255, 0, 255, 255]),
        );

        r.draw_filled_circle(
            &mut img,
            Circle::new((50, 50), 10),
            image::Rgba([0, 255, 0, 255]),
        );

        aar.draw_arc(
            &mut img,
            Circle::new((10, 50), 5),
            Angle::Degrees(0f64),
            Angle::Degrees(90f64),
            image::Rgba([0, 255, 0, 255]),
        );
        aar.draw_arc(
            &mut img,
            Circle::new((10, 50), 5),
            Angle::Degrees(90f64),
            Angle::Degrees(180f64),
            image::Rgba([0, 0, 255, 255]),
        );
        aar.draw_arc(
            &mut img,
            Circle::new((10, 50), 5),
            Angle::Degrees(180f64),
            Angle::Degrees(270f64),
            image::Rgba([255, 0, 0, 255]),
        );
        r.draw_arc(
            &mut img,
            Circle::new((10, 50), 5),
            Angle::Degrees(270f64),
            Angle::Degrees(359.9f64),
            image::Rgba([255, 0, 255, 255]),
        );

        r.draw_filled_arc(
            &mut img,
            Circle::new((10, 80), 10),
            Angle::Degrees(0f64),
            Angle::Degrees(90f64),
            image::Rgba([0, 255, 0, 255]),
        );
        aar.draw_filled_arc(
            &mut img,
            Circle::new((10, 80), 10),
            Angle::Degrees(90f64),
            Angle::Degrees(180f64),
            image::Rgba([0, 0, 255, 255]),
        );
        aar.draw_filled_arc(
            &mut img,
            Circle::new((10, 80), 10),
            Angle::Degrees(180f64),
            Angle::Degrees(270f64),
            image::Rgba([255, 0, 0, 255]),
        );
        aar.draw_filled_arc(
            &mut img,
            Circle::new((10, 80), 10),
            Angle::Degrees(270f64),
            Angle::Degrees(359.9999f64),
            image::Rgba([255, 0, 255, 255]),
        );

        aar.draw_rounded_rect(
            &mut img,
            Rect::new(80, 10, 10, 10),
            3,
            image::Rgba([255, 0, 0, 255]),
        );
        aar.draw_filled_rounded_rect(
            &mut img,
            Rect::new(80, 25, 10, 10),
            3,
            image::Rgba([255, 0, 0, 255]),
        );

        img.save("./test.png").unwrap();
    }
}
