use piston_window::*;
use rand::Rng;
use crossterm::{QueueableCommand, cursor, terminal};

use std::io::stdout;


use crate::{pixel::*};
use crate::draw::*;
use crate::elements::*;

pub struct Simulation {
    pub size: [u32; 2],
    pub scale: u32,
    pub window: PistonWindow,
    pub grid: Vec<Vec<Pixel>>,
    pub gravity: f64,
    pub friction: f64,
    pub mouse_pos: [u32; 2],
    pub edge_mode: bool,
    pub elements: ElementList,
}

impl Simulation {
    pub fn new() -> Simulation {

        let elements = ElementList::new();

        const SCALE: u32 = 10;
        const SIZE: [u32; 2] = [50; 2];
        
        let gravity: f64 = 1.0;
        let friction: f64 = 0.99;        
        let mouse_pos = [0, 0];
        let edge_mode: bool = true;
        
        let mut grid: Vec<Vec<Pixel>> = vec![];
        
        //SET pos in pixels to index in grid
        for y in 0..SIZE[1] {
            let mut row: Vec<Pixel> = vec![];
            for x in 0..SIZE[0] {
                row.push(Pixel::spawn("air".to_string(), [x, y]));
            }
            grid.push(row);
        }
        
        //temp check to make sure air is set

        let window: PistonWindow = WindowSettings::new("Pixel Simulation", [SIZE[0] * SCALE, SIZE[1] * SCALE])
            .exit_on_esc(true)
            .build()
            .unwrap();

        Simulation {
            size: SIZE,
            scale: SCALE,
            window,
            grid,
            gravity,
            friction,
            mouse_pos,
            edge_mode,
            elements,
        }
    }

    pub fn update(&mut self, _verbose: bool) {

        //println!("UPDATE"); 
        self.empty_check(self.grid.clone());
        //pause();

        let mut new_grid = self.grid.clone();
        //get list of pixels ordered by and velocity
        let mut pixel_list = Vec::new();

        for y in 0..self.size[1] {
            for x in 0..self.size[0] {
                let mut pix = self.grid[y as usize][x as usize];
                pix.pos = [x, y];
              
                //add randomness to low force particles
                let move_chance: f64;
                let mut rng = rand::thread_rng();
                if pix.min_force == 0.0 {
                    move_chance = 1.0;
                } else if pix.min_force < 0.5 {
                    move_chance = rng.gen_range(0.0..1.0 - pix.min_force);
                } else {
                    move_chance = 0.0;
                }

                if move_chance > pix.min_force {
                    pix.vel[0] += rng.gen_range(-self.gravity * (2.0 - pix.min_force)..self.gravity * (2.0 - pix.min_force));
                    pix.vel[1] += rng.gen_range(-self.gravity * (1.4 - pix.min_force)..self.gravity * (1.0 - pix.min_force));
                }
                  
                pix.vel = [pix.vel[0] * self.friction * pix.friction_multiplier, pix.vel[1] * self.friction * pix.friction_multiplier];

                pix.vel[1] += pix.gravity_multiplier * self.gravity;
                
                if pix.min_force > pix.vel[0].abs() {
                    pix.vel[0] = 0.0;
                } 
                if pix.min_force > pix.vel[1].abs() {
                    pix.vel[1] = 0.0;
                }
                //println!("PIXEL VEL: {:?}", pix.vel);

                pixel_list.push(pix);
            }
        }

        pixel_list.sort_by(|a, b| ((a.vel[0] + a.vel[1]) * a.density).partial_cmp(&((b.vel[0] + b.vel[1]) * b.density)).unwrap());

        let mut pixel_pairs = Vec::new();

        for pix in &pixel_list {
            let pos = [pix.pos[0] as i32 + pix.vel[0] as i32, pix.pos[1] as i32 + pix.vel[1] as i32];
            let new_pos = wrapped_coord(pos, self.edge_mode, self.size);
            //if no movement, skip
            if pix.ptype == 0 || new_pos == pix.pos || self.grid[new_pos[1] as usize][new_pos[0] as usize].ptype == pix.ptype || pix.density < self.grid[new_pos[1] as usize][new_pos[0] as usize].density {
                //println!("NO MOVE: {:?}", pix.pos);
                continue;
            } else {
                //println!("MOVE: {:?} {:?}", pix.pos, new_pos);           
            }
            //add move to list
            pixel_pairs.push([pix.pos, [new_pos[0] as u32, new_pos[1] as u32]]);
        }
        
        //for every move, swap pixels
        for pair in &pixel_pairs {
            let old_pos = pair[0];
            let new_pos = pair[1];

            //println!("POS SWAP: {:?} {:?}", old_pos, new_pos);

            let mut pix = self.grid[old_pos[1] as usize][old_pos[0] as usize];
            pix.pos = [new_pos[0] as u32, new_pos[1] as u32];

            let mut swap = self.grid[new_pos[1] as usize][new_pos[0] as usize];
            swap.pos = [old_pos[0] as u32, old_pos[1] as u32];

            new_grid[old_pos[1] as usize][old_pos[0] as usize] = swap;
            new_grid[new_pos[1] as usize][new_pos[0] as usize] = pix;
        }

        self.empty_check(new_grid.clone());   

        self.grid = new_grid;

    }

    pub fn empty_check(&self, grid: Vec<Vec<Pixel>>) {
        //temp check, make sure no type 0 pixels
        for y in 0..self.size[1] {
            for x in 0..self.size[0] {
                if grid[y as usize][x as usize].ptype == 0 {
                    println!("ERROR: TYPE 0 PIXEL AT {:?} {:?}", x, y);
                }
            }
        }
    }

    pub fn print(&self, _verbose: bool) {
        let mut stdout = stdout();
        stdout.queue(cursor::SavePosition).unwrap();
        for y in 0..self.size[1] {
            print!("{:04}|", y);
            for x in 0..self.size[0] {
                print!("{}", self.grid[y as usize][x as usize].print());
            }
            println!();
        }
        stdout.queue(cursor::RestorePosition).unwrap();
        stdout.queue(terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();
    }
                
    //place pixel (sand) at mouse position
    pub fn place_pixel(&mut self, typ: String) {
        let x = self.mouse_pos[0];
        let y = self.mouse_pos[1];
        self.grid[y as usize][x as usize] = Pixel::spawn(typ, [x, y]);
    }

    //place pixel (stone) from mouse position to mouse position
    pub fn place_line(&mut self, pos: [u32; 2], typ: String) {
        let rect = rect_pos(self.mouse_pos, pos);
        //println!("{:?}", rect);
        for x in rect[0]..rect[2] + 1{
            for y in rect[1]..rect[3] + 1{
                self.grid[y as usize][x as usize] = Pixel::spawn(typ.clone(), [x, y]);
            }
        }
    }

    //replace air with water below mouse position
    pub fn sea(&mut self, typ: String) {
        let _x = self.mouse_pos[0];
        let y = self.mouse_pos[1];
        for x in 0..self.size[0] {
            for y in y..self.size[1] {
                if self.grid[y as usize][x as usize].density <= 0.3 {
                    self.grid[y as usize][x as usize] = Pixel::spawn(typ.clone(), [x, y]);
                }
            }
        }
    }

    //pixel radius iterator
    pub fn radius_iter(&mut self, r: u32) -> Vec<[u32; 2]> {
        radius(self.mouse_pos, r, self.size)
    }

    //place pixel (air) at mouse position r=radius
    pub fn erase(&mut self, r: u32, _typ: String ) {
        let _x = self.mouse_pos[0];
        let _y = self.mouse_pos[1];
        for pixel in self.radius_iter(r) {
            self.grid[pixel[1] as usize][pixel[0] as usize] = air([pixel[0], pixel[1]]);
        }
    }

    //print grid to console nicely
    //legend
    //[|] = air
    //[8] = sand
    //[O] = water
    //[X] = stone
    //[#] = brick
}

pub fn radius(pos: [u32; 2], r: u32, size: [u32; 2]) -> Vec<[u32; 2]> {
    let x = pos[0];
    let y = pos[1];
    let mut vec = Vec::new();
    for i in 0..r {
        for j in 0..r {
            if i*i + j*j <= r*r {
                if 0 <= x as i32 + (i as i32) && x as i32 + (i as i32) < size[0] as i32 && 0 <= y as i32 + (j as i32) && y as i32 + (j as i32) < size[1] as i32 {
                    vec.push([x + i, y + j]);
                } 
                if 0 <= x as i32 - (i as i32) && x as i32 - (i as i32) < size[0] as i32 && 0 <= y as i32 + (j as i32) && y as i32 + (j as i32) < size[1] as i32 {
                    vec.push([x - i, y + j]);
                }
                if 0 <= x as i32 + (i as i32) && x as i32 + (i as i32) < size[0] as i32 && 0 <= y as i32 - (j as i32) && y as i32 - (j as i32) < size[1] as i32 {
                    vec.push([x + i, y - j]);
                }
                if 0 <= x as i32 - (i as i32) && x as i32 - (i as i32) < size[0] as i32 && 0 <= y as i32 - (j as i32) && y as i32 - (j as i32) < size[1] as i32 {
                    vec.push([x - i, y - j]);
                }
            }
        }
    }
    vec
}

pub fn wrapped_coord(pos: [i32; 2], edge_mode: bool, size: [u32; 2]) -> [u32; 2] {
    let bx = size[0] as i32 - 1;
    let by = size[1] as i32 - 1;
    let mut x = pos[0];
    let mut y = pos[1];
    if edge_mode {
        //go to other side of grid
        if x < 0 {
            x = bx;
        } else if x > bx {
            x = 0;
        }
        if y < 0 {
            y = by;
        } else if y > by {
            y = 0;
        }
    } else {
        //stay on same side of grid
        if x < 0 {
            x = 0;
        } else if x > bx {
            x = bx;
        }
        if y < 0 {
            y = 0;
        } else if y > by {
            y = by;
        }
    }
    [x as u32, y as u32]            
}

pub fn _adjacents(pos: [u32; 2], edge_mode: bool, size: [u32; 2]) -> Vec<[u32; 2]> {
    let mut vec = Vec::new();
    
    for i in -1..2 {
        for j in -1..2 {
            if i != 0 || j != 0 {
                let x = pos[0] as i32 + i;
                let y = pos[1] as i32 + j;
                let coord = wrapped_coord([x, y], edge_mode, size);
                if coord != pos {
                    vec.push(coord);
                }
            }
        }
    }
    vec
}