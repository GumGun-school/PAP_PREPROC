
use std::collections::HashMap;

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

const GROUPS:[u8;34] = [0,0,0,0,/**/1,1,1,1,1,1,1,2,2,2,2,2,2,3,3,3,3,/*vegetation*/4,1,5,6,6,7,7,7,7,7,7,7,8];

pub fn convert(data: &u32) -> [u8; 4] {
    let mut res = [0; 4];
    res[..].copy_from_slice(&data.to_le_bytes());
    res[3]=255;
    res
}


pub fn fill_cluster() -> HashMap<u8, (u8, image::Rgba<u8>)> {
    
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    use rand::RngCore;
    
    let mut rng = StdRng::seed_from_u64(121212u64);
    let color_vec:Vec<_> = (0..30).map(|_|{image::Rgba::<u8>(convert(&(rng.next_u32())))}).collect();
    let mut color_vec_iter = color_vec.iter();
    
    let mut clusters: HashMap<u8, (u8, image::Rgba<u8>)> = HashMap::new();
    let mut helper: HashMap<u8, image::Rgba<u8>> = HashMap::new();
    
    for (index, cat) in GROUPS.iter().enumerate() {
        match helper.get(cat) {
            Some(class) => {
                clusters.insert(index.try_into().unwrap(), (*cat, *class));
            }
            None => {
                let pixel_holder = image::Rgba::<u8>(convert(&(rng.next_u32())));
                clusters.insert(index.try_into().unwrap(), (*cat, pixel_holder));
                helper.insert(*cat, pixel_holder);
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
