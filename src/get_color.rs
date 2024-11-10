use super::*;
use image::GrayImage;
use image::RgbaImage;
use std::fs::File;

pub fn dir_crawl_color(dataset_base:&Path) {
    let mut og_data = PathBuf::new();
    let mut new_dir = PathBuf::new();
    og_data.push(&dataset_base);
    
    og_data.push("gtFine");
    let _ = std::fs::create_dir(&new_dir);
    
    let re = Regex::new(".*labelIds.*").unwrap();
    let mut context = Context{regex:re, current:0, found_img:0, clusters: HashMap::new(), local_clusters: HashMap::new()};
    
    
    let mut clusters: HashMap<u8, image::Rgba<u8>> = HashMap::default();
    //panic!("{:#?}", clusters);
    
    'limit: for mode_entry in fs::read_dir(og_data).unwrap() {
        let mode_dir = mode_entry.unwrap();
        if !mode_dir.file_type().unwrap().is_dir() {
            continue;
        }
        let old_local_path = mode_dir.path();
        let folder_name = old_local_path.file_name().unwrap();
            
        if folder_name != "test" {
            for city_folder in fs::read_dir(old_local_path).unwrap() {
                let city_entry = city_folder.unwrap();
                if !city_entry.file_type().unwrap().is_dir() {
                    continue;
                }
                let old_city_path = city_entry.path();
                let city_name = old_city_path.file_name().unwrap();
                
                for image_entry in fs::read_dir(old_city_path).unwrap() {
                    let image_file = image_entry.unwrap();
                    if img_prepare(&mut context, image_file, &mut clusters) {
                        break 'limit;
                    }
                } 
                
                new_dir.pop();
            }
        } else {
            println!("TEST");
        }
        new_dir.pop();
    }
    
    save_file(&clusters);
    println!("FINAL");
}

pub fn save_file(clusters:&HashMap<u8, image::Rgba<u8>>){
    let mut file = File::create(format!("{DATASET}/color.meta")).unwrap();
    let mut vector:Vec<u32> = Vec::new();
    println!("cats {:?}", clusters.len());
    println!("{:?}", clusters);
    
    for (key, content) in clusters {
        vector.push(u32::from_be_bytes(content.0));
    }
    
    bincode::serialize_into(file, &vector).unwrap();
}

pub fn img_prepare(context:&mut Context, image_file:DirEntry, clusters:&mut HashMap<u8, image::Rgba<u8>>) -> bool{
    if !image_file.file_type().unwrap().is_file() {
        return false;
    }
    
    let img_path = image_file.path();
    if !context.regex.is_match(img_path.to_str().unwrap()) {
        return false;
    }
    if context.current < SKIP {
        context.current += 1;
        return false;
    }
    
    
    
    let img = ImageReader::open(&img_path).unwrap().decode().unwrap();
    let img_name = img_path.file_name().unwrap().to_str().unwrap();
    println!("\timgNo. {}: {}", context.current+1, img_name);
    let gray_img = img.into_luma8();
    
    
    let mut img_string = img_path.display().to_string();
    img_string.truncate(img_string.len()-12);
    img_string.push_str("color.png");
    println!("\timgNo. {}: {}", context.current+1, img_string);
    let color_img = ImageReader::open(&img_string).unwrap().decode().unwrap();
    let color_img = color_img.into_rgba8();
    
    
    let cat_number = context.clusters.len();
    
    if img_get_color(&gray_img, &color_img, clusters) {
        println!("found color\nnew len: {}", clusters.len());
        println!("{:?}", clusters);
    }
    
    context.current += 1;
    
    if let Some(number) = LIMIT {
        if context.current >= number {
            context.print_cluster();
            return true;
        }
    }
    false
}

fn img_get_color(mut grey:&GrayImage, color:&RgbaImage, clusters:&mut HashMap<u8, image::Rgba<u8>>) -> bool {
    
    let (cols, rows) = (grey.width(), grey.height());
    let mut ret = false;
    /*
    eprintln!("{:?} {:?}", image.width(), image.height());
    eprintln!("{:?} {:?}", img.width(), img.height());
    */
    for row in 0..cols {
        for col in 0..rows {
            let pixel = grey.get_pixel(row, col).0[0];
            match clusters.get(&pixel) {
                Some(_) => {
                }
                None => {
                    let color = color.get_pixel(row, col).clone();
                    clusters.insert(pixel, color);
                    ret = true;
                }
            }
        }
    }
    ret
}
