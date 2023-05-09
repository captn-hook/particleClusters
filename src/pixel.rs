use piston_window::*;

use crate::draw::*;

use crate::elements::*;

#[derive(Clone, Copy)]
pub struct Pixel {
    pub ptype: u8,
    pub pos: [u32; 2],
    pub vel: [f64; 2],
    pub color: [f32; 4],
    pub density: f64,
    pub min_force: f64,
    pub gravity_multiplier: f64,
    pub friction_multiplier: f64,
}

impl Pixel {
    pub fn new(ptype: u8, pos: [u32; 2], vel: [f64; 2], color: [f32; 4], density: f64, min_force: f64, gravity_multiplier: f64, friction_multiplier: f64) -> Pixel {
        Pixel {
            ptype,
            pos,
            vel,
            color,
            density,
            min_force,
            gravity_multiplier,
            friction_multiplier,    
        }
    }

    pub fn spawn(typ: String, pos: [u32; 2]) -> Pixel {
        if typ == "air" {
            air(pos)
        } else if typ == "sand" {
            sand(pos)
        } else if typ == "water" {
            water(pos)
        } else if typ == "lava" {
            lava(pos)
        } else if typ == "stone" {
            stone(pos)
        } else if typ == "brick" {
            brick(pos)
        } else if typ == "wood" {
            wood(pos)
        } else if typ == "smoke" {
            smoke(pos)
        } else if typ == "glass" {
            glass(pos)
        } else {
            Pixel::default()
        }
    }

    pub fn default() -> Pixel {
        //null pixel, should only be used to instantiate empty grid, then replaced
        Pixel {
            ptype: 0,
            pos: [0, 0],
            vel: [0.0, 0.0],
            color: [1.0, 0.0, 1.0, 1.0],
            density: 0.0,
            min_force: 0.0,
            gravity_multiplier: 0.0,
            friction_multiplier: 0.0,
        }
    }

    // pub fn print(&self) -> String {
    //     let direct = false;
    //     let symbols = " ░▒▓█EDCBAX";
    //     //use density to determine symbol, and color from color
    //     let mut printstr = format!("[{}]", symbols.chars().nth(self.density as usize).unwrap());
        
    //     //get max from colored rgb
    //     //red max
    //     if self.color[0] > self.color[1] && self.color[0] > self.color[2] {
    //         printstr = printstr.red().to_string();
    //     //green max 
    //     } else if self.color[1] > self.color[0] && self.color[1] > self.color[2] {
    //         printstr = printstr.green().to_string();
    //     //blue max
    //     } else if self.color[2] > self.color[0] && self.color[2] > self.color[1] {
    //         printstr = printstr.blue().to_string();
    //     //all equal
    //     } else {
    //         printstr = printstr.white().to_string();
    //     }

    //     //get min from colored rgb
    //     //red min
    //     if self.color[0] < self.color[1] && self.color[0] < self.color[2] {
    //         printstr = printstr.on_magenta().to_string();
    //     //green min
    //     } else if self.color[1] < self.color[0] && self.color[1] < self.color[2] {
    //         printstr = printstr.on_purple().to_string();
    //     //blue min
    //     } else if self.color[2] < self.color[0] && self.color[2] < self.color[1] {
    //         printstr = printstr.on_cyan().to_string();
    //     //all equal
    //     } else {
    //         printstr = printstr.on_white().to_string();
    //     }


    //     if direct {
    //         print!("{}", printstr);
    //     }
    //     return printstr.to_string();
    // }

    pub fn draw(&self, scale: u32) -> ([f64; 4], [f32; 4]) {
        
        let coords = screem(self.pos, scale);
        let tx = coords[0];
        let ty = coords[1];

        return (coords, self.color)
    }
}

pub fn pixel_draw(pixels: &Vec<Vec<Pixel>>, context: Context, graphics: &mut G2d, scale: u32) {
    for (_y, row) in pixels.iter().enumerate() {
        for (_x, pixel) in row.iter().enumerate() {
            
            let mut ruler = false;
                    
            if pixel.density > 0.95 && (pixel.vel[0] == 0.0 && pixel.vel[1] == 0.0) {
                ruler = true;
            }

            let (coords, color) = pixel.draw(scale);

            let square = rectangle::square(coords[0], coords[1], scale as f64);

            rectangle(color, square, context.transform, graphics);

            if ruler {
                //draw outline
                draw_cursor_outline(coords, context, graphics);
            }
        }
    }
}
