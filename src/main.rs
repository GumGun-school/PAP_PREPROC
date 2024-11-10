use std::path::Path;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::DirEntry;
use std::fs;
use image::ImageReader;
use regex::Regex;

mod categories;
mod interpret;
use interpret::*;
mod process;
use process::*;
mod get_color;


struct Context{
    regex: Regex,
    current: usize,
    clusters: HashMap<u8, usize>,
    local_clusters: HashMap<u8, usize>,
    found_img: usize,
}

enum Mode{
    Analysis,
    Process,
    Color,
}


const MODE:Mode = Mode::Process;
const SEARCH:Option<u8> = None;
const IMG_TO_FIND:usize = 5;
const SKIP:usize = 0;//2975;
const LIMIT:Option<usize> = None;//Some(SKIP+70);

const DATASET:&str = "/home/gumgun/pap/datasets/";


impl Context{
    fn print_cluster(&self) {
        println!("\t\tcluster:cats {}", self.clusters.len());
        for (key, content) in self.clusters.iter(){
            let cat_name = categories::Cat(*key);
            
            println!("\t\t{:?}:\t{}:\t{:?}", key, cat_name.to_str(), content);
        }
    }
    fn print_local_cluster(&self) {
        println!("\t\tcluster:cats {}", self.local_clusters.len());
        for (key, content) in self.local_clusters.iter(){
            println!("\t\t{:?}: {:?}", key, content);
        }
    }
}

fn main() {
    let dataset_base = Path::new(DATASET);
    match MODE {
        Mode::Process => {
            dir_crawl_process(&dataset_base);
        }
        Mode::Analysis => {
            dir_crawl_interpret(&dataset_base);
        }
        Mode::Color => {
            get_color::dir_crawl_color(&dataset_base);
        }
    }
    
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

