extern crate image;

use std::fs::File;
use std::path::Path;
use image::GenericImage;

fn print_info() {
    println!("Image Splitter");
    println!("Splits a source image into multiple rectilinear tiles.");
    println!("Usage: image-splitter <file-name> <tile-width> <tile-height>");
    println!();
    println!("For example: To split a 100 x 100 source image named \"cats.png\" into 25 20x20 png files, use the command");
    println!("\timage-splitter cats.png 20 20");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        print_info();
        std::process::exit(0);
    }
    let input_file_name = &args[1];
    let output_width = args[2].parse::<u32>().unwrap();
    let output_height = args[3].parse::<u32>().unwrap();
    let file_path = Path::new(input_file_name);
    let input_image = {
        match image::open(&file_path) {
            Ok(input_image) => input_image,
            Err(image_error) => {
                println!("{}", image_error);
                std::process::exit(0);
            }
        }    
    };

    let (input_width, input_height) = input_image.dimensions();
    if input_width < output_width || input_height < output_height {
        print_info();
        println!("\nInvalid arguments.");
        println!("Output dimensions cannot be larger than size of input image.");
        std::process::exit(0);
    }

    let columns = input_width / output_width;
    let rows = input_height / output_height;


    for i in 0 .. columns {
        for j in 0 .. rows {
            let src_x = i * output_width;
            let src_y = j * output_height;

            let mut output_buffer = image::ImageBuffer::new(output_width, output_height);
            for x in 0 .. output_width {
                for y in 0 .. output_height {
                    let in_pixel = input_image.get_pixel(src_x + x, src_y + y);
                    output_buffer.put_pixel(x, y, in_pixel);
                }
            }

            let tile_index = j * columns + i;
            let output_stem  = file_path.file_stem().unwrap_or(&std::ffi::OsStr::new("tile")).to_str().unwrap();
            let output_name = format!("{}_{:04}.png", output_stem, tile_index);
            let output_file = &mut File::create(&Path::new(&output_name)).unwrap();
            let _ = image::ImageRgba8(output_buffer).save(output_file, image::PNG);
        }
    }

}
