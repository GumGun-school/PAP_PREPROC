use super::*;
use image::GrayImage;




pub fn dir_crawl_process(dataset_base:&Path) {
    let mut og_data = PathBuf::new();
    let mut new_dir = PathBuf::new();
    og_data.push(&dataset_base);
    new_dir.push(&dataset_base);
    
    og_data.push("gtFine");
    new_dir.push("gtFineMod");
    let _ = std::fs::create_dir(&new_dir);
    
    let re = Regex::new(".*labelIds.*").unwrap();
    let mut context = Context{regex:re, current:0, found_img:0, clusters: HashMap::new(), local_clusters: HashMap::new()};
    
    
    let mut clusters: HashMap<u8, (u8, image::Rgba<u8>)> = categories::fill_cluster();
    //panic!("{:#?}", clusters);
    
    for mode_entry in fs::read_dir(og_data).unwrap() {
        let mode_dir = mode_entry.unwrap();
        if !mode_dir.file_type().unwrap().is_dir() {
            continue;
        }
        let old_local_path = mode_dir.path();
        let folder_name = old_local_path.file_name().unwrap();
        new_dir.push(folder_name);
        let _ = std::fs::create_dir(&new_dir);
            
        if folder_name != "test" {
            for city_folder in fs::read_dir(old_local_path).unwrap() {
                let city_entry = city_folder.unwrap();
                if !city_entry.file_type().unwrap().is_dir() {
                    continue;
                }
                let old_city_path = city_entry.path();
                let city_name = old_city_path.file_name().unwrap();
                new_dir.push(&city_name);
                let _ = std::fs::create_dir(&new_dir);
                
                println!("directory:\n{:?}", new_dir);
                for image_entry in fs::read_dir(old_city_path).unwrap() {
                    let image_file = image_entry.unwrap();
                    img_prepare(&mut context, image_file, &mut clusters, &new_dir);
                } 
                
                new_dir.pop();
            }
        } else {
            
            println!("TEST");
        }
        new_dir.pop();
    }
    println!("FINAL");
}

pub fn img_prepare<Q>(context:&mut Context, image_file:DirEntry, clusters:&mut HashMap<u8, (u8, image::Rgba<u8>)>, save_path:&Q) 
where Q:AsRef<Path>{
    if !image_file.file_type().unwrap().is_file() {
        return;
    }
    let img_path = image_file.path();
    if !context.regex.is_match(img_path.to_str().unwrap()) {
        return;
    }
    if context.current < SKIP {
        context.current += 1;
        return;
    }
    
    let img = ImageReader::open(&img_path).unwrap().decode().unwrap();
    let img_name = img_path.file_name().unwrap().to_str().unwrap();
    println!("\timgNo. {}: {}", context.current+1, img_name);
    let mut gray_img = img.into_luma8();
    
    let cat_number = context.clusters.len();
    context.local_clusters.clear();
    
    
    let _ = img_manip_bw(&mut gray_img, clusters, save_path, img_name);
    
    context.current += 1;
    
    if let Some(number) = LIMIT {
        if context.current >= number {
            context.print_cluster();
            panic!();
        }
    }
}

pub fn convert(data: &u32) -> [u8; 4] {
    let mut res = [0; 4];
    res[..].copy_from_slice(&data.to_le_bytes());
    res[3]=255;
    res
}

fn img_manip_bw<Q>(mut grey:&mut GrayImage, clusters:&mut HashMap<u8, (u8, image::Rgba<u8>)>, save_path:Q, save_name:&str) -> Result<(), ()> 
where Q:AsRef<Path>{
    
    let (cols, rows) = (grey.width(), grey.height());
    let mut color = image::RgbaImage::new(cols, rows);
    /*
    eprintln!("{:?} {:?}", image.width(), image.height());
    eprintln!("{:?} {:?}", img.width(), img.height());
    */
    for row in 0..cols {
        for col in 0..rows {
            let pixel = grey.get_pixel(row, col).0[0];
            let (new_label, new_color) = clusters.get(&pixel).unwrap();
            grey.put_pixel(row, col, image::Luma::<u8>([*new_label]));
            color.put_pixel(row, col, *new_color);
            
        }
    }
    //dbg!(&clusters);
    
    let mut holder = save_name.to_string();
    
    let grey_path = <Path>::join(save_path.as_ref(), &holder);
    grey.save(grey_path).map_err(|_|())?;
    
    holder.truncate(holder.len()-12);
    holder.push_str("color.png");
    
    let color_path = <Path>::join(save_path.as_ref(), &holder);
    color.save(color_path).map_err(|_|())?;
    Ok(())
}


