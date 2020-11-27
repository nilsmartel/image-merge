fn get_input_files() -> impl Iterator<Item = String> {
    std::env::args().skip(1)
}

fn main() {
    let images = get_input_files()
        .map(|name| {
            use image::open;

            println!("opening image {}", &name);
            let img = open(&name).expect(&format!("failed to open image {}", &name));
            println!("done");
            img
        })
        // assert that all images are rgba8
        .map(to_rgba8);

    let length = get_input_files().fold(0.0, |n, _| n + 1.0);
    let full_image = add_images(images, length as usize).div(length).to_rgb8();

    let filename = simple_user_input::get_input("please specify file name");
    println!("Saving file as {}", &filename);

    full_image.save(filename).expect("Failed to save file");
}

fn add_images(mut images: impl Iterator<Item = image::RgbImage>, total: usize) -> Rgb32Image {
    // create black image, where we can add up all other images to
    let first = images.next().unwrap();

    let dim = first.dimensions();
    let mut dest = Rgb32Image::new(dim);
    let iter = std::iter::once(first);
    for (i, image) in iter.chain(images).enumerate() {
        assert_eq!(dest.dim, image.dimensions());

        for (index, pixel) in image.pixels().enumerate() {
            let [r, g, b] = pixel.0;
            let pxl = &mut dest.data[index];
            pxl.r += r as u32;
            pxl.g += g as u32;
            pxl.b += b as u32;
        }

        println!("Processed image {:03} of {}", i, total);
    }

    dest
}

struct Rgb32Image {
    dim: (u32, u32),
    data: Vec<Rgb32>,
}

impl Rgb32Image {
    fn new(dim: (u32, u32)) -> Self {
        let size = (dim.0 * dim.1) as usize;
        let data = vec![Rgb32::black(); size];
        Self { dim, data }
    }

    fn div(mut self, value: f32) -> Self {
        assert_ne!(value, 0.0);
        // TODO improve performance using simd
        for mut pixel in self.data.iter_mut() {
            let (mut r, mut g, mut b) = (pixel.r as f32, pixel.g as f32, pixel.b as f32);
            r /= value;
            g /= value;
            b /= value;

            pixel.r = r as u32;
            pixel.g = g as u32;
            pixel.b = b as u32;
        }

        self
    }

    fn to_rgb8(self) -> image::RgbImage {
        let mut img = image::RgbImage::new(self.dim.0, self.dim.1);
        for (dest, src) in img.pixels_mut().zip(self.data.into_iter()) {
            let src: Rgb32 = src;
            let data = [src.r as u8, src.g as u8, src.b as u8];
            let rgb: image::Rgb<u8> = image::Rgb(data);
            *dest = rgb;
        }

        img
    }
}

#[derive(Copy, Clone)]
struct Rgb32 {
    r: u32,
    g: u32,
    b: u32,
}

impl Rgb32 {
    fn black() -> Self {
        Rgb32 { r: 0, g: 0, b: 0 }
    }
}

fn to_rgba8(image: image::DynamicImage) -> image::RgbImage {
    match image {
        image::DynamicImage::ImageRgb8(img) => img,
        image::DynamicImage::ImageRgba8(img) => {
            let (w, h) = img.dimensions();
            let mut buffer: image::RgbImage = image::ImageBuffer::new(w, h);
            for (dest, src) in buffer.pixels_mut().zip(img.pixels()) {
                let src: image::Rgb<u8> = image::Rgb([src[0], src[1], src[2]]);
                *dest = src;
            }

            buffer
        }
        _ => {
            eprintln!("Expect image to be in rgb8 format");
            std::process::exit(1)
        }
    }
}

// stolen from https://users.rust-lang.org/t/how-to-get-user-input/5176/8
mod simple_user_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String {
        println!("{}", prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {}
            Err(_no_updates_is_fine) => {}
        }
        input.trim().to_string()
    }
}
