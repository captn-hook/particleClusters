use crate::pixel::*;
use crate::draw::*;

//singletons of each element string and their number u8
pub struct ElementList {
    pub elements: Vec<String>,
    pub element_codes: Vec<u8>,
    
    _lenght: usize,
}

impl ElementList {
    pub fn new() -> ElementList {
        let elements = vec![
            "default".to_string(),
            "air".to_string(),
            "sand".to_string(),
            "water".to_string(),
            "lava".to_string(),
            "stone".to_string(),
            "brick".to_string(),
            "wood".to_string(),
            "smoke".to_string(),
            "glass".to_string(),
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
        ElementList {
            elements,
            element_codes,
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

    pub fn _get_name(&self, index: u8) -> String {
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
    let r = rand_color(0.5, 0.1);
    let g = rand_color(0.5, 0.1);
    let b = rand_color(0.6, 0.2);

    Pixel::new(9,
        pos,
        [0.0; 2],
        [r, g, b, 1.0],
        0.4,
        0.45,
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
        0.4,
        0.45,
        1.2,
        0.99,
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
        0.01,
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