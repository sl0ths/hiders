mod transform;
use crate::transform::*;
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use std::fs::File;
use std::io::{BufReader, Lines};

#[allow(dead_code)]
fn print_matrix<T: std::fmt::Display>(vec: &[T]) {
    for i in 0..8 {
        for j in 0..8 {
            print!("{:03} ", vec[i * 8 + j]);
        }
        print!("\n")
    }
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
    println!("Hello, world!");
}
