use std::collections::HashMap;
use std::collections::HashSet;
use super::DATASET;
use std::fs::File;

#[derive(Copy, Clone, Debug)]
pub struct Cat(pub u8);



impl Cat {
    pub fn to_str(&self) -> &'static str {
        match self.0 {
            0  => "other     ",
            1  => "CAR logo??",
            2  => "border2   ",
            3  => "border    ",
            4  => "static    ",
            5  => "dynamic   ",
            6  => "ground    ",
            7  => "road      ",
            8  => "sidewalk  ",
            9  => "parking   ",
            10 => "rail      ",
            11 => "building  ",
            12 => "wall      ",
            13 => "fence     ",
            14 => "guard rail",
            15 => "bridge    ",
            16 => "tunnel    ",
            17 => "pole      ",
            18 => "polegroup ",
            19 => "traf light",
            20 => "traf sign ",
            21 => "vegetation",
            22 => "terrain   ",
            23 => "sky       ",
            24 => "person    ",
            25 => "rider     ",
            26 => "car       ",
            27 => "truck     ",
            28 => "bus       ",
            29 => "caraban   ",
            30 => "trailer   ",
            31 => "train     ",
            32 => "motorcicle",
            33 => "bicycle   ",
            _ =>  "NOCAT     ",
        }
    }
}

const GROUPS:[u8;34] = [
/*0  => "other     ",*/ 8,
/*1  => "CAR logo??",*/ 8,
/*2  => "border2   ",*/ 8,
/*3  => "border    ",*/ 8,
/*4  => "static    ",*/ 33,
/*5  => "dynamic   ",*/ 33,
/*6  => "ground    ",*/ 11,
/*7  => "road      ",*/ 11,
/*8  => "sidewalk  ",*/ 11,
/*9  => "parking   ",*/ 11,
/*10 => "rail      ",*/ 11,
/*11 => "building  ",*/ 12,
/*12 => "wall      ",*/ 12,
/*13 => "fence     ",*/ 12,
/*14 => "guard rail",*/ 12,
/*15 => "bridge    ",*/ 12,
/*16 => "tunnel    ",*/ 12,
/*17 => "pole      ",*/ 13,
/*18 => "polegroup ",*/ 13,
/*19 => "traf light",*/ 13,
/*20 => "traf sign ",*/ 13,
/*21 => "vegetation",*/ 21,
/*22 => "terrain   ",*/ 11,
/*23 => "sky       ",*/ 28,
/*24 => "person    ",*/ 7,
/*25 => "rider     ",*/ 7,
/*26 => "car       ",*/ 17,
/*27 => "truck     ",*/ 17,
/*28 => "bus       ",*/ 17,
/*29 => "caraban   ",*/ 17,
/*30 => "trailer   ",*/ 17,
/*31 => "train     ",*/ 17,
/*32 => "motorcicle",*/ 17,
/*33 => "bicycle   ",*/ 20];
/*_ =>  "NOCAT     ",*/

pub fn fill_cluster() -> HashMap<u8, (u8, image::Rgba<u8>)> {
    
    let mut file = File::open(format!("{DATASET}/color.meta")).unwrap();
    let color_vec:Vec<u32> = bincode::deserialize_from(file).unwrap();
    println!("len: {}\n{:?} ", color_vec.len(), color_vec);
    let mut color_set:HashSet<_> = color_vec.iter().map(|a|image::Rgba::<u8>(a.to_be_bytes())).collect();
    
    println!("len: {}\n{:?} ", color_set.len(), color_set);
    
    let mut color_iter = color_set.iter();
    
    let mut clusters: HashMap<u8, (u8, image::Rgba<u8>)> = HashMap::new();
    let mut helper: HashMap<u8, image::Rgba<u8>> = HashMap::new();

    for (index, cat) in GROUPS.into_iter().enumerate() {
        match helper.get(&cat) {
            Some(class) => {
                clusters.insert(index.try_into().unwrap(), (cat, *class));
            }
            None => {
                let color_holder = *color_iter.next().unwrap();
                clusters.insert(index.try_into().unwrap(), (cat, color_holder));
                helper.insert(cat, color_holder);
            }
        }
    }
    clusters
}



/*
----- road
----- sidewalk
----- parking
----- rail track
----- person
----- rider
----- car
----- truck
----- bus
----- on rails
----- motorcycle
----- bicycle
----- caravan
----- trailer
----- building
----- wall
----- fence
----- guard rail
----- bridge
----- tunnel
----- pole
----- pole group
----- traffic sign
----- traffic light
----- vegetation
----- terrain
----- sky
----- ground
----- dynamic
----- static
*/
