use bmp::{open, Image, Pixel};

fn draw_pixel(mut image: Image, x: u32, y: u32, pixel: Pixel) -> Image {
    // technically a useless function and we should use the method instead
    // mostly used to show how to compose these together
    image.set_pixel(x, y, pixel);
    image
}

fn draw_nth_rows(mut image: Image) -> Image {
    let green = Pixel::new(0, 255, 0);
    for column in (0..image.get_height()).step_by(2) {
        for row in 0..image.get_width() {
            image = draw_pixel(image, row, column, green);
        }
    }
    image
}

fn main() {
    let path = std::env::args().nth(1).expect("You must provide a path.");
    let operation = std::env::args().nth(2).expect("You must provide an operation.");
    let size_x = std::env::args().nth(3).unwrap_or("100".to_string());
    let size_y = std::env::args().nth(4).unwrap_or("100".to_string());
    let mut image = Image::new(
        size_x.parse().expect("size_x should be a number"),
        size_y.parse().expect("size_y should be a number")
    );

    if open(path.as_str()).is_err_and(|e| e.kind.as_ref() != "BMP Error") {
        panic!("the file you pointed to is not a bmp")
    }

    image = match operation.as_str() {
        "pixel" => {
            draw_pixel(image, 50, 50, Pixel::new(255, 255, 255))
        },
        "tutor_exercise" => {
            draw_nth_rows(image)
        },
        // Add more image types here in this way - for bonus points, make an enum
        _ => {
            panic!("The operation {operation} was not recognised!");
        }
    };

    image.save(path).expect("the path given should be writable to");
}
