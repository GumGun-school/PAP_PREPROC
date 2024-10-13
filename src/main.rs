use std::path::Path;
use image::ImageReader;
use std::ops::Deref;
use std::ops::DerefMut;
use std::collections::HashMap;
use image::DynamicImage;


fn main() {
    /*
    println!("Hello, world!");
    let path = Path::new("/home/gumgun/pap/datasets/gtFine/train/buh/aachen_000000_000019_gtFine_instanceIds.png");
    let img = ImageReader::open(path).unwrap().decode().unwrap();
    dbg!(img.width(), img.height());
    */
    let og_path = Path::new("/home/gumgun/pap/datasets/gtFine/train/buh/aachen_000000_000019_gtFine_labelIds.png");
    let mut img = ImageReader::open(og_path).unwrap().decode().unwrap();
    
    let path = Path::new("/home/gumgun/pap/datasets/gtFine/train/buh/chng/bw_base.png");
    if let DynamicImage::ImageLuma8(ref mut luma_img) = img {
        let holder = img_manip_bw(luma_img, path).unwrap();
        println!(" categories {}", holder);
    }
    
    
    let og_path = Path::new("/home/gumgun/pap/datasets/gtFine/train/buh/aachen_000000_000019_gtFine_color.png");
    let mut img = ImageReader::open(og_path).unwrap().decode().unwrap();
    
    let path = Path::new("/home/gumgun/pap/datasets/gtFine/train/buh/chng/color_base.png");
    if let DynamicImage::ImageRgba8(ref mut luma_img) = img {
        let holder = img_manip_color(luma_img, path).unwrap();
        println!(" categories {}", holder);
    }
    
    /*
    let mut img_rgba = img.into_rgba8();
    let path = Path::new("/home/gumgun/pap/datasets/gtFine/train/buh/color_cast.png");
    let classes = img_manip_color(&mut img_rgba, path).unwrap();
    */
    /*
    if let DynamicImage::ImageLuma16(luma_img) = img {
        img_manip_bw(&mut luma_img).unwrap();
        println!("ImageRgba8");
    }
    */
    
    /*
    let path = Path::new("/home/gumgun/pap/datasets/gtFine/train/buh/aachen_000000_000019_gtFine_color.png");
    let img = ImageReader::open(path).unwrap().decode().unwrap();
    dbg!(img.width(), img.height());
    if let DynamicImage::ImageRgba8(mut rgb_img) = img {
        let path = Path::new("/home/gumgun/pap/datasets/gtFine/train/buh/chng_aachen_000000_000019_gtFine_color.png");
        img_manip_color(&mut rgb_img, path).unwrap();
        println!("ImageRgba8");
        panic!();
    }
    */
}

use image::Luma;

fn img_manip_bw<Q>(mut image: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>, path:Q) -> Result<usize, ()> 
where Q:AsRef<Path>{
    let mut state = HashMap::new();
    
    for rows in 0..image.height() {
        for cols in 0..image.width() {
            let pixel = image.get_pixel(cols, rows).0[0];
            match state.get_mut(&pixel) {
                Some(value) => {
                    *value += 1;
                }
                None => {
                    state.insert(pixel, 1);
                }
            }
            //image.put_pixel(cols, rows, Luma::<u16>([0xFFC0CBu16]));
        }
    }
    println!("len {}", state.len());
    dbg!(&state);
    image.save(path).map_err(|_|())?;
    Ok(state.len())
}

fn img_manip_color<Q>(mut image: &mut image::RgbaImage, path:Q) -> Result<usize, ()> 
where Q:AsRef<Path>{
    let mut state = HashMap::new();
    
    for rows in 0..image.height() {
        for cols in 0..image.width() {
            
            let pixel = image.get_pixel(cols, rows).clone();
            match state.get_mut(&pixel) {
                Some(value) => {
                    *value += 1;
                }
                None => {
                    state.insert(pixel, 1);
                }
            }
            //image.put_pixel(cols, rows, image::Rgba([255,192,203,u8::MAX]));
        }
    }
    println!("len {}", state.len());
    dbg!(&state);
    image.save(path).unwrap();//.map_err(|_|())?;
    Ok(state.len())
}


