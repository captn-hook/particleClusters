use rand::Rng;
use piston_window::*;
use colored::Colorize;

use crate::draw::*;

use crate::elements::*;

//use crate::Simulation;

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
    pub fn new(typ: String, pos: [u32; 2], vel: [f64; 2], color: [f32; 4], density: f64, min_force: f64, gravity_multiplier: f64, friction_multiplier: f64, element_list: &mut ElementList) -> Pixel {
        let ptype = element_list.get(typ);
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

    pub fn default() -> Pixel {
        air([0, 0])
    }

    pub fn print(&self) -> String {
        let direct = false;
        let symbols = "░▒▓█OEDCBAX";
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
            air(pos)
        }
    }

    pub fn get_update(&mut self, gravity: f64, friction: f64, size: [u32; 2], edge_mode: bool) -> [u32; 2] {
        let mut poscp = self.pos;
        let mut velcp = self.vel;
        velcp[1] += gravity * self.gravity_multiplier;
        velcp[0] *= friction * self.friction_multiplier;
        velcp[1] *= friction * self.friction_multiplier;
        poscp[0] += velcp[0].round() as u32;
        poscp[1] += velcp[1].round() as u32;

        if edge_mode {
            //edge true, bounce off edges
            if poscp[0] < 0 {
                poscp[0] = 0;
                velcp[0] *= -1.0;
            } else if poscp[0] > size[0] - 1 {
                poscp[0] = size[0] - 1;
                velcp[0] *= -1.0;
            } 
            if poscp[1] < 0 {
                poscp[1] = 0;
                velcp[1] *= -1.0;
            } else if poscp[1] > size[1] - 1 {
                poscp[1] = size[1] - 1;
                velcp[1] *= -1.0;
            }
        } else {
            //edge false, wrap around edges
            if poscp[0] < 0 {
                poscp[0] = size[0] - 1;
            } else if poscp[0] > size[0] - 1 {
                poscp[0] = 0;
            } 
            if poscp[1] < 0 {
                poscp[1] = size[1] - 1;
            } else if poscp[1] > size[1] - 1 {
                poscp[1] = 0;
            }
        }

        poscp        
    }

    pub fn update(&mut self, gravity: f64, friction: f64, size: [u32; 2], edge_mode: bool) -> [u32; 2] {
        println!("vel1: {:?}", self.vel);
        self.vel[1] += gravity * self.gravity_multiplier;
        self.vel[0] *= friction * self.friction_multiplier;
        self.vel[1] *= friction * self.friction_multiplier;
        println!("vel2: {:?}", self.vel);
        //velocity safety check, if close to 0, set to 0, if too high, cap
        if self.vel[0] < 0.01 && self.vel[0] > -0.01 {
            self.vel[0] = 0.0;
        } else if self.vel[0] > 100.0 {
            self.vel[0] = 100.0;
        } else if self.vel[0] < -100.0 {
            self.vel[0] = -100.0;
        }
        if self.vel[1] < 0.01 && self.vel[1] > -0.01 {
            self.vel[1] = 0.0;
        } else if self.vel[1] > 100.0 {
            self.vel[1] = 100.0;
        } else if self.vel[1] < -100.0 {
            self.vel[1] = -100.0;
        }
        //if vel is nan, set to 0
        if self.vel[0].is_nan() {
            self.vel[0] = 0.0;
        }
        if self.vel[1].is_nan() {
            self.vel[1] = 0.0;
        }
        
        self.pos[0] += self.vel[0].round() as u32;
        self.pos[1] += self.vel[1].round() as u32;

        println!("SERVING SIZE AND POS: {:?} {:?}", size, self.pos);

        if edge_mode {
            //edge true, bounce off edges'
            println!("IF {:?}", self.pos[0] < 0);
            println!("IF {:?}", self.pos[0] >= size[0]);
            if self.pos[0] < 0 {
                self.pos[0] = 0;
                self.vel[0] *= -1.0;
            } else if self.pos[0] >= size[0] {
                self.pos[0] = size[0] - 1;
                self.vel[0] *= -1.0;
            } 
            if self.pos[1] < 0 {
                self.pos[1] = 0;
                self.vel[1] *= -1.0;
            } else if self.pos[1] >= size[1] {
                self.pos[1] = size[1] - 1;
                self.vel[1] *= -1.0;
            }

            println!("SERVING2: {:?}", self.pos);
            self.pos

        } else {
            //edge false, wrap around edges
            if self.pos[0] < 0 {
                self.pos[0] = size[0] - 1;
            } else if self.pos[0] >= size[0] {
                self.pos[0] = 0;
            }
            if self.pos[1] < 0 {
                self.pos[1] = size[1] - 1;
            } else if self.pos[1] >= size[1] {
                self.pos[1] = 0;
            }

            println!("SERVING1: {:?}", self.pos);
            self.pos
        }
    }

    pub fn collide(&mut self, gravity: f64, friction: f64, size: [u32; 2], edge_mode: bool, other_pixel: &Pixel) -> [u32; 2] {
        //collison with other, update this with others tranferred momentum and collision angle
        let angle = (self.pos[1] as f64 - other_pixel.pos[1] as f64) / (self.pos[0] as f64 - other_pixel.pos[0] as f64);
        self.vel[0] = (other_pixel.vel[0] + other_pixel.vel[1] * angle) / 2.0;
        self.vel[1] = (other_pixel.vel[1] + other_pixel.vel[0] * angle) / 2.0;
        self.update(gravity, friction, size, edge_mode)
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

pub fn pixel_draw(pixels: [[Pixel; 50]; 50], context: Context, graphics: &mut G2d, scale: u32) {
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
