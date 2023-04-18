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

    pub fn wrapped_coord(pos: [i32; 2], wrap: bool) -> [u32; 2] {
        let mut x = pos[0];
        let mut y = pos[1];
        if wrap {
            //go to other side of grid
            if x > 49 {
                x = 0;
            } else if x < 0 {
                x = 49;
            }
            if y > 49 {
                y = 0;
            } else if y < 0 {
                y = 49;
            }
        } else {
            //clamp to edge
            if x > 49 {
                x = 49;
            } else if x < 0 {
                x = 0;
            }
            if y > 49 {
                y = 49;
            } else if y < 0 {
                y = 0;
            }
        }
        [x as u32, y as u32]
    }

    pub fn adjacents(&self) -> [[u32; 2]; 8] {
        let mut adjacent: [[u32; 2]; 8] = [[0; 2]; 8];
        let mut x = self.pos[0];
        let mut y = self.pos[1];

        for ix in 0..3 {
            for iy in 0..3 {
                if ix == 1 && iy == 1 {
                    continue;
                }

                let coord = [(x + ix) as i32, (y + iy) as i32];

                let wrapped = Pixel::wrapped_coord(coord, false);

                adjacent[(ix + iy) as usize] = wrapped;
            }
        }
        adjacent
    }

    fn phys(&mut self, wrap: bool) -> [u32; 2] {
        //apply gravity, friction, etc, return next pos
        
        self.vel[1] += self.gravity_multiplier * self.density;

        self.vel[0] *= self.friction_multiplier;
        self.vel[1] *= self.friction_multiplier;

        let mut next_pos = [(self.pos[0] as f64 + self.vel[0]) as u32, (self.pos[1] as f64 + self.vel[1]) as u32];

        next_pos = Pixel::wrapped_coord([next_pos[0] as i32, next_pos[1] as i32], wrap);
        
        let adjacents = self.adjacents();

        let mut closest = [0, 0];
        let mut dist = 1000000.0;

        for a in adjacents {
            //get the closest adjacent pixel from next_pos
            let x = ((a[0] as i32 - next_pos[0] as i32).abs() as f64).powi(2);
            let y = ((a[1] as i32 - next_pos[1] as i32).abs() as f64).powi(2);
            let d = (x + y).sqrt();
            if d < dist {
                dist = d;
                closest = a;
            }    
        }

        closest
    }

    //ok
    //so/
    //a pixel update cycle shoulde take a pixel COPT and a grid REFERENCE but not disturb the grid
    //the COPY is to be modified at each step, subtracting from its velocity and adding to its position
    //to get final postion for that frame
    //IF THE PIXEL TRAVELS THRU ITS LIKE KINDA AND ENDS AT ITS LIKE KIND,
    //"solid", ignore for now
    //otherwise
    //aPPEND THE COLLISIONS CHAIN
    ////amd tjem
    //fuck
    pub fn update(&mut self, grid: [[Pixel; 50]; 50], gravity: f64, friction: f64, edge_mode: bool, step: [i32; 2]) -> Option<(Pixel, Pixel)> {
        //get result pos
        let pos = self.phys(edge_mode);

        None
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
