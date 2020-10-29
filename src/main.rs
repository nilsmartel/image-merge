use image::GenericImageView;
fn main() {
    // files to open and merge
    let files = std::env::args().skip(1);

    let images = files.map(|name| {
        use image::open;

        println!("opening image {}", &name);
        let img = open(&name).expect(&format!("failed to open image {}", &name));
        println!("done");
        img
    });

    // assert that all images are of type rgb
    let images = images.map(to_rgba8).collect::<Vec<_>>();

    let length = images.len() as f32;

    // dimension of the final image
    let dim = images[0].dimensions();
    // verify all images have the same dimensions
    for img in images.iter().skip(1) {
        assert_eq!(dim, img.dimensions());
    }

    let full_image = add_images(dim, images).div(length).to_rgb8();

    let filename = simple_user_input::get_input("please specify file name");
    println!("Saving file as {}", &filename);

    full_image.save(filename).expect("Failed to save file");
}

fn add_images(dim: (u32, u32), images: Vec<image::RgbImage>) -> Rgb32Image {
    let max = images.len();

    // create black image, where we can add up all other images to
    let mut dest = Rgb32Image::new(dim);
    for (i, image) in images.iter().enumerate() {
        for (index, pixel) in image.pixels().enumerate() {
            let [r, g, b] = pixel.0;
            let pxl = &mut dest.data[index];
            pxl.r += r as u32;
            pxl.g += g as u32;
            pxl.b += b as u32;
        }

        println!("Processed image {:03} of {}", i, max);
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
        _ => {
            eprintln!("Expect image to be in rgb8 format");
            std::process::exit(1)
        }
    }
}

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
