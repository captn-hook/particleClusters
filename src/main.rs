use piston_window::*;
use rand::Rng;

mod pixel;
use pixel::*;

mod simulate;
use simulate::*;

pub struct Simulation {
    pub size: [u32; 2],
    pub scale: u32,
    pub window: PistonWindow,
    pub grid: [[Pixel; 50]; 50],
    pub gravity: f64,
    pub friction: f64,
    pub mouse_pos: [u32; 2],
    pub edge_mode: bool,
}

impl Simulation {
    pub fn new() -> Simulation {
        const scale: u32 = 10;
        const size: [u32; 2] = [50; 2];
        
        let mut gravity: f64 = 0.1;
        let mut friction: f64 = 0.99;        
        let mut mouse_pos = [0, 0];
        let mut edge_mode: bool = true;

        
        let mut grid = [[Pixel::air([0, 0]); size[0] as usize]; size[1] as usize];
        
        //SET pos in pixels to index in grid
        for y in 0..size[1] {
            for x in 0..size[0] {
                grid[y as usize][x as usize].pos = [x, y];
            }
        }
        
        let window: PistonWindow = WindowSettings::new("Pixel Simulation", [size[0] * scale, size[1] * scale])
            .exit_on_esc(true)
            .build()
            .unwrap();

        Simulation {
            size,
            scale,
            window,
            grid,
            gravity,
            friction,
            mouse_pos,
            edge_mode,
        }
    }

    pub fn swap_pixels(&mut self, pos1: [u32; 2], pos2: [u32; 2]) {
        let pixel1 = self.grid[pos1[1] as usize][pos1[0] as usize];
        let pixel2 = self.grid[pos2[1] as usize][pos2[0] as usize];
        self.grid[pos1[1] as usize][pos1[0] as usize] = pixel2;
        self.grid[pos2[1] as usize][pos2[0] as usize] = pixel1;
    }

    pub fn peek(&mut self, pos: [u32; 2]) -> Pixel {
        self.grid[pos[1] as usize][pos[0] as usize]
    }

    //place pixel (sand) at mouse position
    pub fn place_pixel(&mut self) {
        let x = self.mouse_pos[0];
        let y = self.mouse_pos[1];
        self.grid[y as usize][x as usize] = Pixel::sand([x, y]);
    }

    pub fn burn(&mut self) {
        let x = self.mouse_pos[0];
        let y = self.mouse_pos[1];
        self.grid[y as usize][x as usize] = Pixel::lava([x, y]);
    }

    pub fn wall(&mut self) {
        let x = self.mouse_pos[0];
        let y = self.mouse_pos[1];
        self.grid[y as usize][x as usize] = Pixel::brick([x, y]);
    }

    pub fn line_iter(&mut self, pos1: [u32; 2], pos2: [u32; 2]) -> Vec<Pixel> {
        //gets all pixels that a pixel would travel through, until it hits a pixel with a higher density
        let m = (pos2[1] as f64 - pos1[1] as f64) / (pos2[0] as f64 - pos1[0] as f64);
        let b = pos1[1] as f64 - m * pos1[0] as f64;
        let mut vec: Vec<Pixel> = Vec::new();
        let mut x1: u32;
        let mut x2: u32;
        let mut y1: u32;
        let mut y2: u32;

        let density = self.grid[pos1[1] as usize][pos1[0] as usize].density;

        let forwards: bool = pos1[0] < pos2[0];

        if pos1[0] < pos2[0] {
            x1 = pos1[0];
            x2 = pos2[0];
        } else {
            x1 = pos2[0];
            x2 = pos1[0];
        }
        if pos1[1] < pos2[1] {
            y1 = pos1[1];
            y2 = pos2[1];
        } else {
            y1 = pos2[1];
            y2 = pos1[1];
        }
            
        //limit x1 and x2 to grid
        if x1 > self.size[0] {
            x1 = self.size[0];
        } else if x1 < 0 {
            x1 = 0;
        }
        if x2 > self.size[0] {
            x2 = self.size[0];
        } else if x2 < 0 {
            x2 = 0;
        }
        if y1 > self.size[1] {
            y1 = self.size[1];
        } else if y1 < 0 {
            y1 = 0;
        }
        if y2 > self.size[1] {
            y2 = self.size[1];
        } else if y2 < 0 {
            y2 = 0;
        }
        
        for x in x1..x2 {
            let y = (m * x as f64 + b) as u32;
            if y < self.size[1] {
                if self.grid[y as usize][x as usize].density < density {
                    vec.push(self.grid[y as usize][x as usize]);
                } else if self.grid[y as usize][x as usize].density == density {
                    //flip a coin
                    let mut rng = rand::thread_rng();
                    let num: f64 = rng.gen();
                    if num > 0.5 {
                        vec.push(self.grid[y as usize][x as usize]);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if forwards {
            vec.reverse();
        }

        vec
    }

    //place pixel (stone) from mouse position to mouse position
    pub fn place_line(&mut self, pos: [u32; 2]) {
        //four cases: x1 < x2, x1 > x2, y1 < y2, y1 > y2
        let x = self.mouse_pos[0] as i32 - pos[0] as i32;
        let y = self.mouse_pos[1] as i32 - pos[1] as i32;
        let mut x1: u32;
        let mut x2: u32;
        let mut y1: u32;
        let mut y2: u32;

        if x < 0 {
            //println!("x < 0");
            x1 = self.mouse_pos[0];
            x2 = pos[0];
        } else {
            //println!("x > 0");
            x1 = pos[0];
            x2 = self.mouse_pos[0];
        }
        if y < 0 {
            //println!("y < 0");
            y1 = self.mouse_pos[1];
            y2 = pos[1];
        } else {
            //println!("y > 0");
            y1 = pos[1];
            y2 = self.mouse_pos[1];
        }

        if y < 0 && x < 0 {
            //println!("CASE 1");
        } else {
            x2 += 1;
            y2 += 1;
        }
        
        println!("x1: {}, x2: {}, y1: {}, y2: {}", x1, x2, y1, y2);
        for x in x1..x2 {
            for y in y1..y2 {
                self.grid[y as usize][x as usize] = Pixel::stone([x, y]);
            }
        }
    }
    
    //replace air with water below mouse position
    pub fn sea(&mut self) {
        let x = self.mouse_pos[0];
        let y = self.mouse_pos[1];
        for x in 0..self.size[0] {
            for y in y..self.size[1] {
                if self.grid[y as usize][x as usize].density <= 0.3 {
                    self.grid[y as usize][x as usize] = Pixel::water([x, y]);
                }
            }
        }
    }

    //pixel radius iterator
    pub fn radius_iter(&mut self, r: u32) -> Vec<[u32; 2]> {
        let x = self.mouse_pos[0];
        let y = self.mouse_pos[1];
        let mut vec = Vec::new();
        for i in 0..r {
            for j in 0..r {
                if i*i + j*j <= r*r {
                    if 0 <= x as i32 + (i as i32) && x as i32 + (i as i32) < self.size[0] as i32 && 0 <= y as i32 + (j as i32) && y as i32 + (j as i32) < self.size[1] as i32 {
                        vec.push([x + i, y + j]);
                    } 
                    if 0 <= x as i32 - (i as i32) && x as i32 - (i as i32) < self.size[0] as i32 && 0 <= y as i32 + (j as i32) && y as i32 + (j as i32) < self.size[1] as i32 {
                        vec.push([x - i, y + j]);
                    }
                    if 0 <= x as i32 + (i as i32) && x as i32 + (i as i32) < self.size[0] as i32 && 0 <= y as i32 - (j as i32) && y as i32 - (j as i32) < self.size[1] as i32 {
                        vec.push([x + i, y - j]);
                    }
                    if 0 <= x as i32 - (i as i32) && x as i32 - (i as i32) < self.size[0] as i32 && 0 <= y as i32 - (j as i32) && y as i32 - (j as i32) < self.size[1] as i32 {
                        vec.push([x - i, y - j]);
                    }
                }
            }
        }
        vec
    }

    //place pixel (air) at mouse position r=radius
    pub fn erase(&mut self, r: u32) {
        let x = self.mouse_pos[0];
        let y = self.mouse_pos[1];
        for pixel in self.radius_iter(r) {
            self.grid[pixel[1] as usize][pixel[0] as usize] = Pixel::air([pixel[0], pixel[1]]);
        }
    }
}

fn main() {
    let mut sim = Simulation::new();

    let mut left_click = false;
    let mut last_left_click = [0, 0];
    let mut right_click = false;
    let mut middle_click = false;
    let mut space = false;
    let mut shift = false;
    let mut ctrl = false;
    
    while let Some(event) = sim.window.next() {

        //draw on render
        if let Some(args) = event.render_args() {
            new_frame(&mut sim.window, &event, &sim.grid, sim.scale);
        }

        //track mouse position
        if let Some(pos) = event.mouse_cursor_args() {
            //integer and scaled mouse position
            let x = pos[0] as u32 / sim.scale;
            let y = pos[1] as u32 / sim.scale;
            //limit mouse position to grid
            if 0 <= x && x < sim.size[0] && 0 <= y && y < sim.size[1] {
                sim.mouse_pos = [x, y];
                //println!("MOUSE {} {}", x, y);
            }
        }

        //track button holds
        //m down
        if let Some(Button::Mouse(button)) = event.press_args() {
            if button == MouseButton::Left {
                left_click = true;
                //left click to place pixel
                last_left_click = sim.mouse_pos;
            }
            if button == MouseButton::Right {
                right_click = true;
            }
            if button == MouseButton::Middle {
                middle_click = true;
            }
        }
        //m up
        if let Some(Button::Mouse(button)) = event.release_args() {
            if button == MouseButton::Left {
                left_click = false;
                //left click to place pixel
                sim.place_line(last_left_click);
            }
            if button == MouseButton::Right {
                right_click = false;
            }
            if button == MouseButton::Middle {
                middle_click = false;
            }
        }

        //k down
        if let Some(Button::Keyboard(key)) = event.press_args() {
            if key == Key::Space {
                space = true;
            }
            if key == Key::LShift {
                shift = true;
            }
            if key == Key::LCtrl {
                ctrl = true;
            }
        }
        //k up
        if let Some(Button::Keyboard(key)) = event.release_args() {
            if key == Key::Space {
                space = false;
            }
            if key == Key::LShift {
                shift = false;
            }
            if key == Key::LCtrl {
                ctrl = false;
            }
        }

        //middle click to to place sand
        if middle_click {
            sim.place_pixel();
        }
        //right click to erase
        if right_click {
            sim.erase(5);
        }
        //space to place water
        if space {
            sim.sea();
        }
        if shift {
            sim.wall();
        }
        if ctrl {
            sim.burn();
        }


        //update simulation
        if let Some(args) = event.update_args() {
            update(&mut sim, args.dt);
        }
    }
}