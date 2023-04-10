use rand::Rng;
use piston_window::*;

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

impl Pixel {
    pub fn new(pos: [u32; 2], vel: [f64; 2], color: [f32; 4], density: f64, min_force: f64, gravity_multiplier: f64, friction_multiplier: f64) -> Pixel {
        Pixel {
            pos,
            vel,
            color,
            density,
            min_force,
            gravity_multiplier,
            friction_multiplier,
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

    pub fn air(pos: [u32; 2]) -> Pixel {
        let cl = Pixel::rand_color_grey(1.0, 0.05);
        Pixel {
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
            pos,
            vel: [0.0; 2],
            color: [r, g, b, 1.0],
            density: 0.98,
            min_force: 0.0,
            gravity_multiplier: 1.0,
            friction_multiplier: 0.99,
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
        let tx = (self.pos[0] * scale) as f64;
        let ty = (self.pos[1] * scale) as f64;
        let by = ty + scale as f64;
        let lx = tx + scale as f64;
        
        let square = rectangle::square(tx, ty, scale as f64);
        rectangle(self.color, square, context.transform, graphics);

        if ruler {
            //draw outline
            let line = [tx, ty, lx, ty];
            let line2 = [tx, ty, tx, by];
            let line3 = [tx, by, lx, by];
            let line4 = [lx, ty, lx, by];
            for (i, line) in [line, line2, line3, line4].iter().enumerate() {
                let lined = line::Line::new([0.0, 0.0, 0.0, 1.0], 1.0);
                lined.draw(*line, &context.draw_state, context.transform, graphics);
            }
        }
    }
}

pub fn new_frame(window: &mut PistonWindow, event: &Event, pixels: &[[Pixel; 50]; 50], scale: u32) {
    window.draw_2d(event, |context, graphics, _| {
        clear([1.0; 4], graphics);

        for (y, row) in pixels.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                
                let mut ruler = false;
                        
                if pixel.density > 0.95 && (pixel.vel[0] == 0.0 && pixel.vel[1] == 0.0) {
                    ruler = true;
                }
                pixel.draw(context, graphics, scale, ruler);

            }
        }
    });
}
