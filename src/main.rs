mod transform;
use crate::transform::*;
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use std::env;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::path::Path;

#[allow(dead_code)]
fn print_matrix<T: std::fmt::Display>(vec: &[T]) {
    for i in 0..8 {
        for j in 0..8 {
            print!("{:03} ", vec[i * 8 + j]);
        }
        print!("\n")
    }
}

fn find(image: &DynamicImage, vigor: usize, layer: usize) -> Vec<u8> {
    let mut hidden_data = Vec::new();
    let (width, height) = image.dimensions();
    // Access individual pixels
    for y in (0..height).step_by(8) {
        let mut blocks = Vec::with_capacity(vigor);
        for x in (0..width).step_by(8) {
            let mut block: Vec<f32> = Vec::new();
            let mut pixel;

            for j in y..y + 8 {
                for i in x..x + 8 {
                    if i < height && j < width {
                        pixel = image.get_pixel(i, j);
                    } else {
                        pixel = Rgba([0, 0, 0, 0]);
                    }
                    block.push(pixel[layer] as f32);
                }
            }

            let dcteed_block = discrete_cosine_transform(&block);
            for i in 64 - vigor..64 {
                blocks.push(dcteed_block[i]);
            }
        }
        hidden_data.push(blocks);
    }
    let hidden_data = hidden_data
        .into_iter()
        .flatten()
        .map(|f| f.floor() as u8)
        .collect::<Vec<u8>>();
    hidden_data
}

#[allow(dead_code)]
fn hide(image: &mut DynamicImage, vigor: usize, lines: &mut Lines<BufReader<File>>, layer: usize) {
    let (width, height) = image.dimensions();
    for y in (0..height).step_by(8) {
        for x in (0..width).step_by(8) {
            let mut blocks: Vec<Vec<f32>> = Vec::new();
            let mut pixel;

            for k in 0..4 {
                let mut block: Vec<f32> = Vec::new();
                for j in y..y + 8 {
                    for i in x..x + 8 {
                        if i < height && j < width {
                            pixel = image.get_pixel(i, j);
                        } else {
                            pixel = Rgba([0, 0, 0, 0]);
                        }
                        block.push(pixel[k] as f32);
                    }
                }
                blocks.push(block);
            }

            let mut dcteed_block = discrete_cosine_transform(&(blocks[layer]));
            // khchi stuff hnaya
            for i in 64 - vigor..64 {
                if let Ok(number) = lines
                    .next()
                    .unwrap_or(Ok(String::from("69")))
                    .expect("panic yo")
                    .parse::<f32>()
                {
                    dcteed_block[i] = number;
                }
            }
            let undecteed_block = discrete_cosine_transform_inverse(&dcteed_block);

            let block = undecteed_block
                .iter()
                .map(|&x| x.ceil() as u8)
                .collect::<Vec<u8>>();

            for j in 0..8 {
                for i in 0..8 {
                    if i < height && j < width {
                        let index = (j * 8 + i) as usize;
                        let pixel = block.get(index).unwrap();
                        let mut arr: [u8; 4] = [0; 4];
                        for k in 0..4 {
                            let shit = if layer == k {
                                *pixel
                            } else {
                                *blocks[k].get(index).unwrap_or(&0_f32) as u8
                            };
                            arr[k] = shit;
                        }
                        image.put_pixel(x + i, y + j, Rgba(arr));
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let usage = "usage: hiders [find vigor input_img | hide vigor input_img input_txt output_img]";
    let action = args.get(1).expect(usage);
    let vigor: usize = args.get(2).expect(usage).parse().unwrap();
    let input = args.get(3).expect(usage);
    let mut image = open(Path::new(input)).expect("Failed to open image");
    let layer = 0;
    // Get image dimensions
    if action.eq_ignore_ascii_case("find") {
        // finding in image
        let hidden_data = find(&image, vigor, layer);
        println!(
            "hid data: {:?}\nlength = {}",
            hidden_data,
            hidden_data.len()
        );
    } else if action.eq_ignore_ascii_case("hide") {
        // hiding in image
        // hiders hide [text] [in image]
        let data_to_hide = &args.get(4).expect(usage);
        let output = &args.get(5).expect(usage);

        // open the file for reading data to hide
        let file = File::open(data_to_hide).unwrap();
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        hide(&mut image, vigor, &mut lines, layer);
        image.save(output).expect("failed to save");
        println!("image saved")
    } else {
        println!("{usage}");
        return;
    }
}
