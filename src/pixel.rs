use rand::Rng;
use piston_window::*;
use colored::Colorize;

use crate::draw::*;

use crate::elements::*;

use crate::simulate::*;

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

    blocked: bool,
}

#[derive(Clone)]
pub struct PType {
    //each ptype lives in pixel/ptype.rs
    pub name: String,
    pub interact: Vec<String>,
}

impl PType {
    pub fn new(name: String) -> PType {
        
        let interacts: Vec<String> = vec![];
        PType {
            name,
            interact: interacts,
        }
    } 
    
    pub fn add_interact(&mut self, interact: String) {
        self.interact.push(interact);
    }

    pub fn fetch_interact(&self, interact: String) -> bool {
        for i in &self.interact {
            if i == &interact {
                return  true;
            }
        }
        false
    }
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
            blocked: false,
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
            blocked: false,
        }
    }

    pub fn block(&mut self) {
        self.blocked = true;
    }

    pub fn unblock(&mut self) {
        self.blocked = false;
    }

    pub fn is_blocked(&self) -> bool {
        self.blocked
    }

    pub fn update_pos(&mut self, pos: [u32; 2]) {
        self.pos = pos;
    }

    pub fn print(&self) -> String {
        let direct = false;
        let symbols = " ░▒▓█EDCBAX";
        //use density to determine symbol, and color from color
        let mut printstr = format!("[{}]", symbols.chars().nth(self.density as usize).unwrap());
        
        //get max from colored rgb
        //red max
        if self.color[0] > self.color[1] && self.color[0] > self.color[2] {
            printstr = printstr.red().to_string();
        //green max 
        } else if self.color[1] > self.color[0] && self.color[1] > self.color[2] {
            printstr = printstr.green().to_string();
        //blue max
        } else if self.color[2] > self.color[0] && self.color[2] > self.color[1] {
            printstr = printstr.blue().to_string();
        //all equal
        } else {
            printstr = printstr.white().to_string();
        }

        //get min from colored rgb
        //red min
        if self.color[0] < self.color[1] && self.color[0] < self.color[2] {
            printstr = printstr.on_magenta().to_string();
        //green min
        } else if self.color[1] < self.color[0] && self.color[1] < self.color[2] {
            printstr = printstr.on_purple().to_string();
        //blue min
        } else if self.color[2] < self.color[0] && self.color[2] < self.color[1] {
            printstr = printstr.on_cyan().to_string();
        //all equal
        } else {
            printstr = printstr.on_white().to_string();
        }


        if direct {
            print!("{}", printstr);
        }
        return printstr.to_string();
    }

    pub fn draw(&self, context: Context, graphics: &mut G2d, scale: u32, ruler: bool) {
        
        let coords = screem(self.pos, scale);
        let tx = coords[0];
        let ty = coords[1];
        
        let square = rectangle::square(tx, ty, scale as f64);
        rectangle(self.color, square, context.transform, graphics);

        if ruler {
            //draw outline
            draw_cursor_outline(coords, context, graphics);
        }
    }
}

pub fn pixel_draw(pixels: [[Pixel; 5]; 5], context: Context, graphics: &mut G2d, scale: u32) {
    for (_y, row) in pixels.iter().enumerate() {
        for (_x, pixel) in row.iter().enumerate() {
            
            let mut ruler = false;
                    
            if pixel.density > 0.95 && (pixel.vel[0] == 0.0 && pixel.vel[1] == 0.0) {
                ruler = true;
            }
            pixel.draw(context, graphics, scale, ruler);
        }
    }
}
