use std::collections::HashMap;

use crate::pixel::*;
use crate::draw::*;

//singletons of each element string and their number u8
pub struct ElementList {
    pub elements: Vec<String>,
    pub element_codes: Vec<u8>,
    //              input, (catalyst, output)
    pub interactivity: HashMap<u8, Vec<(u8, u8)>>,
    
    _lenght: usize,
}

impl ElementList {
    pub fn new() -> ElementList {
        let elements = vec![
            "default".to_string(), //0
            "air".to_string(),     //1
            "sand".to_string(),    //2
            "water".to_string(),   //3
            "lava".to_string(),    //4
            "stone".to_string(),   //5
            "brick".to_string(),   //6 
            "wood".to_string(),    //7
            "smoke".to_string(),   //8
            "glass".to_string(),   //9
        ];
        let element_codes = vec![
            0,
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
        ];
        let _lenght = elements.len();

        let interactivity_list: [(u8, u8, u8); 5] = [
            //water to air with lava
            (3, 4, 1),
            //lava to stone with water
            (4, 3, 5),
            //lava to smoke with sand
            (4, 2, 8),
            //sand to glass with lava
            (2, 4, 9),
            //wood to smoke with lava
            (7, 4, 8),
        ];

        let mut interactivity: HashMap<u8, Vec<(u8, u8)>> = HashMap::new();

        for i in 0..interactivity_list.len() {
            let (input, catalyst, output) = interactivity_list[i];
            if interactivity.contains_key(&input) {
                interactivity.get_mut(&input).unwrap().push((catalyst, output));
            } else {
                interactivity.insert(input, vec![(catalyst, output)]);
            }
        }

        ElementList {
            elements,
            element_codes,
            interactivity,
            _lenght,
        }
    }

    pub fn _get(&self, index: String) -> u8 {
        for i in 0..self._lenght {
            if self.elements[i] == index {
                return self.element_codes[i];
            }
        }
        return 0;
    }

    pub fn get_name(&self, index: u8) -> String {
        for i in 0..self._lenght {
            if self.element_codes[i] == index {
                return self.elements[i].clone();
            }
        }
        return "default".to_string();
    }

    pub fn _len(&self) -> usize {
        self._lenght
    }

    pub fn _add(&mut self, name: String) {
        self.elements.push(name);
        self.element_codes.push(self._lenght as u8);
        self._lenght += 1;
    }

    pub fn _remove(&mut self, name: String) {
        for i in 0..self._lenght {
            if self.elements[i] == name {
                self.elements.remove(i);
                self.element_codes.remove(i);
                self._lenght -= 1;
                return;
            }
        }
    }
}

pub fn glass(pos: [u32; 2]) -> Pixel {
    let r = rand_color(0.5, 0.03);
    let g = rand_color(0.5, 0.03);
    let b = rand_color(0.65, 0.1);

    Pixel::new(9,
        pos,
        [0.0; 2],
        [r, g, b, 1.0],
        0.8,
        1.1,
        1.2,
        0.99,
    )
}

pub fn smoke(pos: [u32; 2]) -> Pixel {
    let cl = rand_color_grey(0.6, 0.1);

    Pixel::new(8,
        pos,
        [0.0; 2],
        cl,
        0.01,
        0.45,
        -1.5,
        0.95,
    )
}

pub fn wood(pos: [u32; 2]) -> Pixel {
    let r = rand_color(0.5, 0.15);
    let g = rand_color(0.3, 0.05);

    Pixel::new(7,
        pos,
        [0.0; 2],
        [r, g, 0.0, 1.0],
        0.4,
        0.45,
        1.2,
        0.99,
    )
}

pub fn air(pos: [u32; 2]) -> Pixel {
    let cl = rand_color_grey(1.0, 0.05);

    Pixel::new(1,
        pos,
        [0.0; 2],
        cl,
        0.03,
        0.0,
        0.01,
        0.95,
    )
}

pub fn sand(pos: [u32; 2]) -> Pixel {
    let r = rand_color(0.8, 0.15);
    let g = rand_color(0.8, 0.1);

    Pixel::new(2,
        pos,
        [0.0; 2],
        [r, g, 0.0, 1.0],
        0.8,
        0.4,
        1.2,
        0.99,
    )
}

pub fn water(pos: [u32; 2]) -> Pixel {
    let b = rand_color(0.8, 0.15);

    Pixel::new(3,
        pos,
        [0.0; 2],
        [0.0, 0.0, b, 1.0],
        0.5,
        0.0,
        1.0,
        0.99,
    )
}

pub fn lava(pos: [u32; 2]) -> Pixel {
    let r = rand_color(0.8, 0.15);

    Pixel::new(4,
        pos,
        [0.0; 2],
        [r, 0.0, 0.0, 1.0],
        0.9,
        0.2,
        1.0,
        0.95,
    )
}

pub fn stone(pos: [u32; 2]) -> Pixel {
    let cl = rand_color_grey(0.1, 0.25);

    Pixel::new(5,
        pos,
        [0.0; 2],
        cl,
        0.94,
        0.6,
        1.0,
        0.99,
    )
}

pub fn brick(pos: [u32; 2]) -> Pixel {
    let r = rand_color(0.8, 0.15);
    let g = rand_color(0.4, 0.1);
    let b = rand_color(0.2, 0.1);

    Pixel::new(6,
        pos,
        [0.0; 2],
        [r, g, b, 1.0],
        0.98,
        1.1,
        1.0,
        0.99,
    )
}

//fuck ion kno

// #[derive(Clone)]
// pub struct PType {
//     //each ptype lives in pixel/ptype.rs
//     pub name: String,
//     pub interact: Vec<String>,
// }

// impl PType {
//     pub fn _new(name: String) -> PType {
        
//         let interacts: Vec<String> = vec![];
//         PType {
//             name,
//             interact: interacts,
//         }
//     } 
    
//     pub fn _add_interact(&mut self, interact: String) {
//         self.interact.push(interact);
//     }

//     pub fn _fetch_interact(&self, interact: String) -> bool {
//         for i in &self.interact {
//             if i == &interact {
//                 return  true;
//             }
//         }
//         false
//     }
// }