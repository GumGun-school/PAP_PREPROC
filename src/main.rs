use std::path::Path;
use std::ops::Deref;
use std::ops::DerefMut;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use std::fs::DirEntry;
use image::ImageReader;
use image::Luma;
use image::DynamicImage;
use regex::Regex;

struct Context{
    regex: Regex,
    current: usize,
    skip: usize,
    clusters: HashMap<u8, usize>,
    local_clusters: HashMap<u8, usize>,
}

impl Context{
    fn print_cluster(&self) {
        println!("\t\tcluster:cats {}", self.clusters.len());
        for (key, content) in self.clusters.iter(){
            println!("\t\t{:?}: {:?}", key, content);
        }
    }
}

fn main() {
    dir_crawl();
    
    /*
    let og_path = Path::new("/home/gumgun/pap/datasets/gtFine/train/buh/aachen_000000_000019_gtFine_labelIdsSMOL.png");
    let mut img = ImageReader::open(og_path).unwrap().decode().unwrap().into_luma8();
    
    let path = Path::new("/home/gumgun/pap/datasets/gtFine/train/buh/chng/");
    let holder = img_manip_bw(&mut img, path).unwrap();
    */
    
    /*
    match img {
        DynamicImage::ImageLuma8(ref mut luma_img) => {
            let holder = img_manip_bw(luma_img, path).unwrap();
            println!(" categories {}", holder);
        }
        DynamicImage::ImageRgba8(rgba_img) => {
            
            let holder = img_manip_bw(rgba_img, path).unwrap();
            println!(" categories {}", holder);
        }
        _ => {
            panic!("not type");
        }
    }
    */
    
    
    /*
    let og_path = Path::new("/home/gumgun/pap/datasets/gtFine/train/buh/aachen_000000_000019_gtFine_color.png");
    let mut img = ImageReader::open(og_path).unwrap().decode().unwrap();
    
    let path = Path::new("/home/gumgun/pap/datasets/gtFine/train/buh/chng/color_base.png");
    if let DynamicImage::ImageRgba8(ref mut luma_img) = img {
        let holder = img_manip_color(luma_img, path).unwrap();
        println!(" categories {}", holder);
    }
    */
    
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



pub fn convert(data: &u32) -> [u8; 4] {
    let mut res = [0; 4];
    res[..].copy_from_slice(&data.to_le_bytes());
    res[3]=255;
    res
}

fn img_manip_bw<Q>(mut image: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>, path:Q) -> Result<usize, ()> 
where Q:AsRef<Path>{
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    use rand::RngCore;
    
    let mut rng = StdRng::seed_from_u64(121211u64);
    
    let mut clusters: HashMap<u8, (usize, image::Rgba<u8>)> = HashMap::new();
    let color_vec:Vec<_> = (0..30).map(|_|{image::Rgba::<u8>(convert(&(rng.next_u32())))}).collect();
    let mut color_vec_iter = color_vec.iter();
    
    let (cols, rows) = (image.width(), image.height());
    let mut img = image::RgbaImage::new(cols, rows);
    /*
    eprintln!("{:?} {:?}", image.width(), image.height());
    eprintln!("{:?} {:?}", img.width(), img.height());
    */
    for row in 0..cols {
        for col in 0..rows {
            img.put_pixel(row, col, color_vec[0]);
            
            let pixel = image.get_pixel(row, col).0[0];
            
            match clusters.get_mut(&pixel) {
                Some((value, pixel)) => {
                    *value += 1;
                    img.put_pixel(row, col, pixel.clone());
                }
                None => {
                    clusters.insert(pixel, (1, color_vec_iter.next().unwrap().clone()));
                }
            }
        }
    }
    //dbg!(&clusters);
    
    let mut holder = PathBuf::from("bw_base.png");
    let image_path = <Path>::join(path.as_ref(), holder);
    image.save(image_path).map_err(|_|())?;
    
    let mut holder = PathBuf::from("color_base.png");
    let color_path = <Path>::join(path.as_ref(), holder);
    img.save(color_path).map_err(|_|())?;
    
    Ok(clusters.len())
}

fn img_interpret(context:&mut Context, image_file:DirEntry) {
    if !image_file.file_type().unwrap().is_file() {
        return;
    }
    if context.current < context.skip {
        return;
    }
    let img_path = image_file.path();
    if !context.regex.is_match(img_path.to_str().unwrap()) {
        return;
    }
    println!("\timgNo. {}: {}", context.current+1, img_path.file_name().unwrap().to_str().unwrap());
    let img = ImageReader::open(img_path).unwrap().decode().unwrap();
    let gray_img = img.into_luma8();
    
    let cat_number = context.clusters.len();
    context.local_clusters.clear();
    
    img_analysis(&gray_img, &mut context.clusters, &mut context.local_clusters);
    
    if context.local_clusters.len()>18 {
        println!("\t\tlots of cats: {}-{:?}", context.local_clusters.len(), &image_file);
        if context.local_clusters.len()>20 {
            panic!();
        }
    }
    
    if cat_number<context.clusters.len() {
        println!("\t\tfile{:?}", &image_file);
        println!("\t\told categories: {cat_number}");
        println!("\t\tnnew categories: {}", context.clusters.len());
        context.print_cluster();
    }
    context.current += 1;
    //
    if context.current>1000 {
        context.print_cluster();
        panic!();
    }
}

fn img_analysis(image:&image::GrayImage, clusters:&mut HashMap<u8, usize>, local_clusters:&mut HashMap<u8, usize>){
    let (cols, rows) = (image.width(), image.height());
    for row in 30..cols-30 {
        for col in 30..rows-30 {
            let pixel = image.get_pixel(row, col).0[0];
            /*
            if pixel==0 {
            }
            */
            //insert_augment(clusters, pixel);
            insert_augment(local_clusters, pixel);
        }
    }
}


fn img_manip_color<Q>(mut image: &mut image::RgbaImage, path:Q) -> Result<usize, ()> 
where Q:AsRef<Path>{
    let mut clusters = HashMap::new();
    
    for rows in 0..image.height() {
        for cols in 0..image.width() {
            let pixel = image.get_pixel(cols, rows).clone();
            //insert_augment(&mut clusters, pixel);
            match clusters.get_mut(&pixel) {
                Some(value) => {
                    *value += 1;
                }
                None => {
                    clusters.insert(pixel, 1);
                }
            }
            
            //image.put_pixel(cols, rows, image::Rgba([255,192,203,u8::MAX]));
        }
    }
    println!("len {}", clusters.len());
    dbg!(&clusters);
    image.save(path).unwrap();//.map_err(|_|())?;
    
    
    Ok(clusters.len())
}


fn dir_crawl() {
    let dataset_base = Path::new("/home/gumgun/pap/datasets/");
    let mut og_data = PathBuf::new();
    let mut new_dir = PathBuf::new();
    og_data.push(&dataset_base);
    new_dir.push(&dataset_base);
    
    og_data.push("gtFine");
    new_dir.push("gtFineMod");
    std::fs::create_dir(&new_dir);
    
    let re = Regex::new(".*labelIds.*").unwrap();
    let mut context = Context{regex:re, current:0, skip:0, clusters: HashMap::new(), local_clusters: HashMap::new()};
    
    
    let mut clusters: HashMap<u8, usize> = HashMap::new();
    
    for mode_entry in fs::read_dir(og_data).unwrap() {
        let mode_dir = mode_entry.unwrap();
        if !mode_dir.file_type().unwrap().is_dir() {
            continue;
        }
        let old_local_path = mode_dir.path();
        new_dir.push(old_local_path.file_name().unwrap());
        std::fs::create_dir(&new_dir);
        
        for city_folder in fs::read_dir(old_local_path).unwrap() {
            let city_entry = city_folder.unwrap();
            if !city_entry.file_type().unwrap().is_dir() {
                continue;
            }
            let old_city_path = city_entry.path();
            new_dir.push(old_city_path.file_name().unwrap());
            std::fs::create_dir(&new_dir);
            
            println!("directory:\n{:?}", new_dir);
            for image_entry in fs::read_dir(old_city_path).unwrap() {
                let image_file = image_entry.unwrap();
                img_interpret(&mut context, image_file)
                
            } 
            new_dir.pop();
        }
        new_dir.pop();
    }
}

fn insert_augment(hldr: &mut HashMap<u8, usize>, key:u8) -> bool {
    match hldr.get_mut(&key) {
        Some(value) => {
            *value += 1;
            false
        }
        None => {
            hldr.insert(key, 1);
            true
        }
    }
}
    
