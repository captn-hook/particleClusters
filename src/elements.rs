use crate::pixel::*;
use crate::draw::*;

//singletons of each element string and their number u8
pub struct ElementList {
    pub elements: Vec<String>,
    pub element_codes: Vec<u8>,
    
    length: usize,
}

impl ElementList {
    pub fn new() -> ElementList {
        let elements = vec![
            "air".to_string(),
            "sand".to_string(),
            "water".to_string(),
            "lava".to_string(),
            "stone".to_string(),
            "brick".to_string(),
        ];
        let element_codes = vec![
            0,
            1,
            2,
            3,
            4,
            5,
        ];
        let length = elements.len();
        ElementList {
            elements,
            element_codes,
            length,
        }
    }

    pub fn get(&self, index: String) -> u8 {
        for i in 0..self.length {
            if self.elements[i] == index {
                return self.element_codes[i];
            }
        }
        return 0;
    }

    pub fn get_name(&self, index: u8) -> String {
        for i in 0..self.length {
            if self.element_codes[i] == index {
                return self.elements[i].clone();
            }
        }
        return "air".to_string();
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn add(&mut self, name: String) {
        self.elements.push(name);
        self.element_codes.push(self.length as u8);
        self.length += 1;
    }

    pub fn remove(&mut self, name: String) {
        for i in 0..self.length {
            if self.elements[i] == name {
                self.elements.remove(i);
                self.element_codes.remove(i);
                self.length -= 1;
                return;
            }
        }
    }
}

pub fn air(pos: [u32; 2]) -> Pixel {
    let cl = rand_color_grey(1.0, 0.05);
    // Pixel {
    //     ptype: 0,
    //     pos,
    //     vel: [0.0; 2],
    //     color: cl,
    //     density: 0.01,
    //     min_force: 0.0,
    //     gravity_multiplier: 0.01,
    //     friction_multiplier: 0.95,
    //     blocked: false,
    // }
    Pixel::new(0,
        pos,
        [0.0; 2],
        cl,
        0.01,
        0.0,
        0.01,
        0.95,
    )
}

pub fn sand(pos: [u32; 2]) -> Pixel {
    let r = rand_color(0.8, 0.15);
    let g = rand_color(0.8, 0.1);
    // Pixel {
    //     ptype: 1,
    //     pos,
    //     vel: [0.0; 2],
    //     color: [r, g, 0.0, 1.0],
    //     density: 0.8,
    //     min_force: 0.01,
    //     gravity_multiplier: 1.2,
    //     friction_multiplier: 0.99,
    //     blocked: false,
    // }
    Pixel::new(1,
        pos,
        [0.0; 2],
        [r, g, 0.0, 1.0],
        0.8,
        0.01,
        1.2,
        0.99,
    )
}

pub fn water(pos: [u32; 2]) -> Pixel {
    let b = rand_color(0.8, 0.15);
    // Pixel {
    //     ptype: 2,
    //     pos,
    //     vel: [0.0; 2],
    //     color: [0.0, 0.0, b, 1.0],
    //     density: 0.5,
    //     min_force: 0.0,
    //     gravity_multiplier: 1.0,
    //     friction_multiplier: 0.99,
    //     blocked: false,
    // }
    Pixel::new(2,
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
    // Pixel {
    //     ptype: 3,
    //     pos,
    //     vel: [0.0; 2],
    //     color: [r, 0.0, 0.0, 1.0],
    //     density: 0.9,
    //     min_force: 0.0,
    //     gravity_multiplier: 1.0,
    //     friction_multiplier: 0.95,
    //     blocked: false,
    // }
    Pixel::new(3,
        pos,
        [0.0; 2],
        [r, 0.0, 0.0, 1.0],
        0.9,
        0.0,
        1.0,
        0.95,
    )
}

pub fn stone(pos: [u32; 2]) -> Pixel {
    let cl = rand_color_grey(0.1, 0.25);
    // Pixel {
    //     ptype: 4,
    //     pos,
    //     vel: [0.0; 2],
    //     color: cl,
    //     density: 0.94,
    //     min_force: 0.0,
    //     gravity_multiplier: 1.0,
    //     friction_multiplier: 0.99,
    //     blocked: false,
    // }
    Pixel::new(4,
        pos,
        [0.0; 2],
        cl,
        0.94,
        0.0,
        1.0,
        0.99,
    )
}

pub fn brick(pos: [u32; 2]) -> Pixel {
    let r = rand_color(0.8, 0.15);
    let g = rand_color(0.4, 0.1);
    let b = rand_color(0.2, 0.1);
    // Pixel {
    //     ptype: 5,
    //     pos,
    //     vel: [0.0; 2],
    //     color: [r, g, b, 1.0],
    //     density: 0.98,
    //     min_force: 0.0,
    //     gravity_multiplier: 1.0,
    //     friction_multiplier: 0.99,
    //     blocked: false,
    // }
    Pixel::new(5,
        pos,
        [0.0; 2],
        [r, g, b, 1.0],
        0.98,
        0.0,
        1.0,
        0.99,
    )
}