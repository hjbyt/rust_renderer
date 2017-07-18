use std::ops::{Index, IndexMut};
use color::{Color, BLACK};
use image::{ImageBuffer, Rgb};

pub struct ColorImage {
    pub pixels: Vec<Color>,
    pub height: u32,
    pub width: u32,
}

impl ColorImage {
    pub fn new(width: u32, height: u32) -> ColorImage {
        let size = (height * width) as usize;
        ColorImage {
            pixels: vec![BLACK; size],
            height: height,
            width: width,
        }
    }

    pub fn to_image_buffer(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        ImageBuffer::<Rgb<u8>, Vec<u8>>::from_fn(self.width, self.height, |x, y| {
            let (r, g, b) = self[(x as usize, y as usize)].bytes();
            Rgb([r, g, b])
        })
    }
}

impl Index<(usize, usize)> for ColorImage {
    type Output = Color;

    fn index(&self, index: (usize, usize)) -> &Color {
        let (x, y) = index;
        &self.pixels[(y * self.width as usize + x)]
    }
}

impl IndexMut<(usize, usize)> for ColorImage {
    fn index_mut<'a>(&'a mut self, index: (usize, usize)) -> &'a mut Color {
        let (x, y) = index;
        &mut self.pixels[(y * self.width as usize + x)]
    }
}
