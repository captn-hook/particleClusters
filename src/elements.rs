use crate::pixel::*;

pub fn air(pos: [u32; 2]) -> Pixel {
    let cl = Pixel::rand_color_grey(1.0, 0.05);
    Pixel {
        ptype: PType::new("air".to_string()),
        pos,
        vel: [0.0; 2],
        color: cl,
        density: 0.01,
        min_force: 0.0,
        gravity_multiplier: 0.01,
        friction_multiplier: 0.95,
    }
}

pub fn sand(pos: [u32; 2]) -> Pixel {
    let r = Pixel::rand_color(0.8, 0.15);
    let g = Pixel::rand_color(0.8, 0.1);
    Pixel {
        ptype: PType::new("sand".to_string()),
        pos,
        vel: [0.0; 2],
        color: [r, g, 0.0, 1.0],
        density: 0.8,
        min_force: 0.01,
        gravity_multiplier: 1.2,
        friction_multiplier: 0.99,
    }
}

pub fn water(pos: [u32; 2]) -> Pixel {
    let b = Pixel::rand_color(0.8, 0.15);
    Pixel {
        ptype: PType::new("water".to_string()),
        pos,
        vel: [0.0; 2],
        color: [0.0, 0.0, b, 1.0],
        density: 0.5,
        min_force: 0.0,
        gravity_multiplier: 1.0,
        friction_multiplier: 0.99,
    }
}

pub fn lava(pos: [u32; 2]) -> Pixel {
    let r = Pixel::rand_color(0.8, 0.15);
    Pixel {
        ptype: PType::new("lava".to_string()),
        pos,
        vel: [0.0; 2],
        color: [r, 0.0, 0.0, 1.0],
        density: 0.9,
        min_force: 0.0,
        gravity_multiplier: 1.0,
        friction_multiplier: 0.95,
    }
}

pub fn stone(pos: [u32; 2]) -> Pixel {
    let cl = Pixel::rand_color_grey(0.1, 0.25);
    Pixel {
        ptype: PType::new("stone".to_string()),
        pos,
        vel: [0.0; 2],
        color: cl,
        density: 0.94,
        min_force: 0.0,
        gravity_multiplier: 1.0,
        friction_multiplier: 0.99,
    }
}

pub fn brick(pos: [u32; 2]) -> Pixel {
    let r = Pixel::rand_color(0.8, 0.15);
    let g = Pixel::rand_color(0.4, 0.1);
    let b = Pixel::rand_color(0.2, 0.1);
    Pixel {
        ptype: PType::new("brick".to_string()),
        pos,
        vel: [0.0; 2],
        color: [r, g, b, 1.0],
        density: 0.98,
        min_force: 0.0,
        gravity_multiplier: 1.0,
        friction_multiplier: 0.99,
    }
}