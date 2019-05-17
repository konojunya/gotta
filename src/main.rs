extern crate image;
extern crate rand;

use rand::Rng;
use std::fs::File;
use std::path::Path;

struct Board {
    width: u32,
    height: u32,
    buff: Vec<u8>,
}

struct BoardParameter {
    k1: f32,
    k2: f32,
    g: u8,
}

impl Board {
    fn new(width: u32, height: u32) -> Board {
        Board {
            width: width,
            height: height,
            buff: vec![0; (width * height) as usize],
        }
    }

    fn seed(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.buff.iter().count() {
            self.buff[i] = rng.gen();
        }
    }

    fn value(&self, x: u32, y: u32) -> u8 {
        self.buff[(y * self.width + x) as usize]
    }

    fn set_value(&mut self, x: u32, y: u32, value: u8) {
        self.buff[(y * self.width + x) as usize] = value
    }

    fn copy_buff(&mut self, other: &Board) {
        for (i, v) in other.buff.iter().enumerate() {
            self.buff[i] = *v;
        }
    }

    fn image(&self, filename: &str) {
        let mut imgbuf = image::ImageBuffer::new(self.width, self.height);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            *pixel = image::Luma([self.value(x, y)]);
        }
        let ref mut fout = File::create(&Path::new(filename)).unwrap();
        let _ = image::ImageLuma8(imgbuf).save(fout, image::PNG);
    }

    fn neighborhood(&self, x: u32, y: u32) -> [u8; 8] {
        let x1: u32 = (x + self.width - 1) % self.width;
        let x2: u32 = (x + 1) % self.width;
        let y1: u32 = (y + self.height - 1) % self.height;
        let y2: u32 = (y + 1) % self.height;
        [
            self.value(x1, y1),
            self.value(x, y1),
            self.value(x2, y1),
            self.value(x1, y),
            self.value(x2, y),
            self.value(x1, y2),
            self.value(x, y2),
            self.value(x2, y2),
        ]
    }

    fn count_infected(&self, x: u32, y: u32) -> u8 {
        let mut count: u8 = 0;
        for value in &self.neighborhood(x, y) {
            if (*value > 0) & (*value < 255) {
                count += 1;
            }
        }
        count
    }

    fn count_illed(&self, x: u32, y: u32) -> u8 {
        let mut count: u8 = 0;
        for value in &self.neighborhood(x, y) {
            if *value == 255 {
                count += 1;
            }
        }
        count
    }

    fn sum(&self, x: u32, y: u32) -> u16 {
        self.neighborhood(x, y)
            .iter()
            .fold(0u16, |sum, v| sum + *v as u16)
            + self.value(x, y) as u16
    }

    fn step(&mut self, params: &BoardParameter) {
        let mut next_board = Board::new(self.width, self.height);
        let mut value: u8;
        for y in 0..self.height {
            for x in 0..self.width {
                value = self.value(x, y);
                if value == 255 {
                    next_board.set_value(x, y, 0);
                } else {
                    let c_infected = self.count_infected(x, y) as f32;
                    let c_illed = self.count_illed(x, y) as f32;
                    let mut next_value: u16;
                    if value == 0 {
                        let n1 = (c_infected / params.k1).floor() as u16;
                        let n2 = (c_illed / params.k2).floor() as u16;
                        next_value = n1 + n2;
                    } else {
                        let sum = self.sum(x, y) as f32;
                        next_value = (sum / c_infected).floor() as u16 + params.g as u16;
                    }
                    if next_value > 255 {
                        next_value = 255;
                    }
                    next_board.set_value(x, y, next_value as u8)
                }
            }
        }
        self.copy_buff(&next_board);
    }
}

fn main() {
    let mut b = Board::new(200, 160);
    let params = BoardParameter {
        k1: 2.0,
        k2: 3.0,
        g: 3,
    };
    b.seed();
    for i in 0..480 {
        let s = format!("png/foo-{:04}.png", i);
        b.image(&s);
        b.step(&params);
    }
}
