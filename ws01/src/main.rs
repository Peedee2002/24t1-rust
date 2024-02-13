use bmp::{Image, Pixel};

fn main() {
    let path = std::env::args().nth(1).expect("You must provide a path.");
    let operation = std::env::args().nth(2).expect("You must provide an operation.");

    if operation.as_str() == "pixel" {
        draw_pixel(path.as_str());
    } else if operation.as_str() == "tutor_excersise" {
        draw_nth_rows(path.as_str());
    } else {
        eprintln!("The operation {operation} was not recognised!");
    }
}

fn draw_pixel(path: &str) {
    let mut image = Image::new(100, 100); // this keeps being duplicated!! There must be a better waaaaaaaaaaaaaaay 😭
    image.set_pixel(50, 50, Pixel::new(255, 255, 255));
    image.save(path).expect("This should save correctly."); // why am i forced to delegate the task of saving to this function? Surely everyone wants to save their masterpiece 😱
}

fn draw_nth_rows(path: &str) {
    let mut image = Image::new(100, 100);
    // I wish there was a way to get draw_pixel to help me draw lines. There must be a better waaaaaaaaaaaaaaay 😭
    for i in 0..50 {
        for j in 0..100 {
            image.set_pixel(i * 2, j, Pixel::new(0, 255, 0));
        }
    }
    image.save(path).expect("This should save correctly."); 
}
