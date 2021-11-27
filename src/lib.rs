use types::{Circle, Rect};

mod antialiased;
mod basic;
mod types;

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

        img.save("./test.png").unwrap();
    }
}
