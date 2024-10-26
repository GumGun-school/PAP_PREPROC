use super::*;

pub fn dir_crawl_interpret(dataset_base:&Path) {
    let mut og_data = PathBuf::new();
    og_data.push(&dataset_base);
    og_data.push("gtFine");
    
    let re = Regex::new(".*labelIds.*").unwrap();
    let mut context = Context{regex:re, current:0, found_img:0, clusters: HashMap::new(), local_clusters: HashMap::new()};
    
    for mode_entry in fs::read_dir(og_data).unwrap() {
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
                
                println!("directory:\n{:?}", old_city_path);
                for image_entry in fs::read_dir(old_city_path).unwrap() {
                    let image_file = image_entry.unwrap();
                    img_interpret(&mut context, image_file)
                } 
                
            }
        } else {
            //test is not evaled 
            
            println!("TEST");
        }
    }
    println!("FINAL");
    context.print_cluster();
}


pub fn img_interpret(context:&mut Context, image_file:DirEntry) {
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
    
    println!("\timgNo. {}: {}", context.current+1, img_path.file_name().unwrap().to_str().unwrap());
    let img = ImageReader::open(img_path).unwrap().decode().unwrap();
    let gray_img = img.into_luma8();
    
    let cat_number = context.clusters.len();
    context.local_clusters.clear();
    
    if img_analysis(&gray_img, &mut context.clusters, &mut context.local_clusters) {
        context.found_img += 1;
        if context.found_img >= IMG_TO_FIND {
            panic!("found {IMG_TO_FIND} images with pixel {:?}", SEARCH);
        }
        context.print_local_cluster();
    }
    
    if cat_number<context.clusters.len() {
        println!("SEARCHKEY");
        println!("\t\tfile{:?}", &image_file);
        println!("\t\told categories: {cat_number}");
        println!("\t\tnnew categories: {}", context.clusters.len());
        context.print_cluster();
    }
    
    context.current += 1;
    
    if let Some(number) = LIMIT {
        if context.current >= number {
            context.print_cluster();
            panic!();
        }
    }
}

pub fn img_analysis(image:&image::GrayImage, clusters:&mut HashMap<u8, usize>, local_clusters:&mut HashMap<u8, usize>) -> bool {
    let mut pixel_sep = 0;
    let mut ret = false;
    let (cols, rows) = (image.width(), image.height());
    for row in 0..cols {
        for col in 0..rows {
            
            let pixel = image.get_pixel(row, col).0[0];
            
            if let Some(condition) = SEARCH {
                if condition == pixel {
                    ret = true;
                    if pixel_sep == 0 {
                        println!("\t\tRRRRR");
                    }
                    if pixel_sep%5000 == 0 {
                        
                        println!("\t\tfound pixel {}: at {} {}", pixel, row, col);
                    }
                    pixel_sep += 1;
                }
            }
            insert_augment(clusters, pixel);
            insert_augment(local_clusters, pixel);
        }
    }
    ret
}


