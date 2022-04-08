mod args;
use args::Args;

use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};
use std::{fs::File, io::BufReader};


#[derive(Debug)]
enum ImageDataErrors {
  DifferentImageFormats,
}


struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {

    let buffer_capacity =  height * width * 4;
    let buffer= Vec::with_capacity(buffer_capacity.try_into().unwrap());

    FloatingImage{
      width, 
      height, 
      data: buffer,
      name
    }
  }
}

fn main() -> Result<(), ImageDataErrors> {
  let args = Args::new();
  println!("{:?}", args);
  let (image1, image_format1) = find_image_from_path(args.image_1);
  let (image2, image_format2) = find_image_from_path(args.image_2);

  if image_format1 != image_format2 {
    return Err(ImageDataErrors::DifferentImageFormats);
  }
  let(image1, image2) = standardise_size(image1, image2);
  
  let output = FloatingImage::new(image1.width(), image1.height(), args.output);
  Ok(())
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
  let image_format: ImageFormat = image_reader.format().unwrap();
  let image: DynamicImage = image_reader.decode().unwrap();

  (image, image_format)
}

fn get_smalllest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;

  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smalllest_dimensions(image_1.dimensions(), image_2.dimensions());

  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}


fn combine_iamges(image1:DynamicImage, image2:DynamicImage) -> Vec<u8> {
  let vec_1 = image1.to_rgb8().into_vec();
  let vec_2 = image2.to_rgb8().into_vec();

  alrernate_pixels(vec_1, vec_2);
}


fn alrernate_pixels(vec1: Vec<u8>, vec2: Vec<u8>)-> Vec<u8>{

  let combined_data = vec![0u8; vec1.len()];

  let mut i = 0;

  while i < vec1.len() {
    if i % 8 ==0 {
      combined_data.splice(i..=i+3, set_rgba(&vec1, i, i + 3));
    }else {
      combined_data.splice(i..=i+3, set_rgba(&vec2, i, i + 3));
    }
     i+=4;

  }
  combined_data
}


fn set_rgba( vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8>{
  let mut rgba = Vec::new();
  for i in start..=end {
    let val: u8 = match vec.get(i){
      Some(d) => *d,
      None => panic!("Index out of bounds")
    };
    rgba.push(val);
  }
  rgba

}

