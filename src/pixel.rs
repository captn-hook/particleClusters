use rand::Rng;
use piston_window::*;

use crate::draw::*;

use crate::elements::*;

//use crate::Simulation;

#[derive(Clone, Copy)]
pub struct Pixel {
    pub pos: [u32; 2],
    pub vel: [f64; 2],
    pub color: [f32; 4],
    pub density: f64,
    pub min_force: f64,
    pub gravity_multiplier: f64,
    pub friction_multiplier: f64,
}

pub struct PType {
    //each ptype lives in pixel/ptype.rs
    pub name: String,
    pub interact: Vec<String>,
}

impl PType {
    pub fn new(name: String) -> PType {
        
        let interacts: Vec<String> = [''];
        PType {
            name,
            interact: interacts,
        }
    } 
    
    pub fn add_interact(&mut self, interact: String) {
        self.interact.push(interact);
    }

    pub fn fetch_interact(&self, interact: String) -> bool {
        for i in self.interact {
            if i == interact {
                return  true;
            }
        }
        false
    }
}

impl Pixel {
    pub fn new(typ: String, pos: [u32; 2], vel: [f64; 2], color: [f32; 4], density: f64, min_force: f64, gravity_multiplier: f64, friction_multiplier: f64) -> Pixel {
        ptype = PType::new(typ);
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

    pub fn print(&self) {
        if self.density == 0.0 {
            print!("[ ]");
        } else if self.density < 0.1 {
            print!("{}", "[░]".color(self.color));
        } else if self.density < 0.2 {
            print!("{}", "[▒]".color(self.color));
        } else if self.density < 0.3 {
            print!("{}", "[▓]".color(self.color));
        } else if self.density < 0.4 {
            print!("{}", "[█]".color(self.color));
        } else if self.density < 0.5 {
            print!("{}", "[O]".color(self.color));            
        } else if self.density < 0.6 {
            print!("{}", "[E]".color(self.color));            
        } else if self.density < 0.7 {
            print!("{}", "[D]".color(self.color));            
        } else if self.density < 0.8 {
            print!("{}", "[C]".color(self.color));            
        } else if self.density < 0.9 {
            print!("{}", "[B]".color(self.color));            
        } else if self.density < 1.0 {
            print!("{}", "[A]".color(self.color));            
        } else {
            print!("{}", "[X]".color(self.color));            
        }
    }

    pub fn spawn(typ: String, pos: [u32; 2]) -> Pixel {
        if typ == "air" {
            return air(pos);
        } else if typ == "sand" {
            return sand(pos);
        } else if typ == "water" {
            return water(pos);
        } else if typ == "lava" {
            return lava(pos);
        } else if typ == "stone" {
            return stone(pos);
        } else if typ == "brick" {
            brick(pos);
        } else {
            return air(pos);
        }
    }
        

    fn rand_color_margin(color: [f32; 4], margin: f32) -> [f32; 4] {
        let mut rng = rand::thread_rng();
        let mut new_color = color;
        for i in 0..3 {
            new_color[i] = rng.gen_range((color[i] - margin)..(color[i] + margin));
        }
        //max and min
        for i in 0..3 {
            if new_color[i] > 1.0 {
                new_color[i] = 1.0;
            }
            if new_color[i] < 0.0 {
                new_color[i] = 0.0;
            }
        }
        new_color[3] = 1.0;
        new_color
    }

    fn rand_color_grey(value: f32, margin: f32) -> [f32; 4] {
        let mut rng = rand::thread_rng();
        let mut new_color = [rng.gen_range((value - margin)..(value + margin)); 4];

        //max and min
        for i in 0..3 {
            if new_color[i] > 1.0 {
                new_color[i] = 1.0;
            }
            if new_color[i] < 0.0 {
                new_color[i] = 0.0;
            }
        }
        new_color[3] = 1.0;
        new_color
    }

    fn rand_color(value: f32, margin: f32) -> f32 {
        let mut rng = rand::thread_rng();
        rng.gen_range((value - margin)..(value + margin))
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
