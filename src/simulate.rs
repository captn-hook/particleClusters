use piston_window::*;
use rand::Rng;
use crossterm::{QueueableCommand, cursor, terminal, ExecutableCommand};

use std::{thread, time};
use std::io::{Write, stdout};

use colored::Colorize;

use crate::pixel::*;
use crate::draw::*;
use crate::elements::*;

pub struct Simulation {
    pub SIZE: [u32; 2],
    pub SCALE: u32,
    pub window: PistonWindow,
    pub grid: [[Pixel; 5]; 5],
    pub gravity: f64,
    pub friction: f64,
    pub mouse_pos: [u32; 2],
    pub edge_mode: bool,
    pub elements: ElementList,
}

impl Simulation {
    pub fn new() -> Simulation {

        let elements = ElementList::new();

        const SCALE: u32 = 120;
        const SIZE: [u32; 2] = [5; 2];
        
        let mut gravity: f64 = 0.1;
        let mut friction: f64 = 0.99;        
        let mut mouse_pos = [0, 0];
        let mut edge_mode: bool = false;
        
        let mut grid = [[Pixel::default(); 5]; 5];
        
        //SET pos in pixels to index in grid
        for y in 0..SIZE[1] {
            for x in 0..SIZE[0] {
                grid[y as usize][x as usize] = Pixel::spawn("air".to_string(), [x, y]);

            }
        }

        let window: PistonWindow = WindowSettings::new("Pixel Simulation", [SIZE[0] * SCALE, SIZE[1] * SCALE])
            .exit_on_esc(true)
            .build()
            .unwrap();

        Simulation {
            SIZE,
            SCALE,
            window,
            grid,
            gravity,
            friction,
            mouse_pos,
            edge_mode,
            elements,
        }
    }

    pub fn update(&mut self, verbose: bool) {
        let mut new_grid = [[Pixel::default(); 5]; 5];
        //get list of pixels ordered by and velocity
        let mut pixel_list = Vec::new();

        for y in 0..self.SIZE[1] {
            for x in 0..self.SIZE[0] {
                let mut pix = self.grid[y as usize][x as usize];
                pix.unblock();
                pix.pos = [x, y];

                pix.vel = [pix.vel[0] * self.friction, pix.vel[1] * self.friction];

                pix.vel[1] += pix.gravity_multiplier * self.gravity;

                pixel_list.push(pix);
            }
        }

        let mut pixel_pairs = Vec::new();

        for pix in &pixel_list {
            let mut new_pos = [pix.pos[0] as i32 + pix.vel[0] as i32, pix.pos[1] as i32 + pix.vel[1] as i32];
            new_pos = wrapped_coord(new_pos, self.edge_mode, self.SIZE);
            //if no movement, skip
            if new_pos == [pix.pos[0] as i32, pix.pos[1] as i32] {
                continue;
            }
            //add move to list
            pixel_pairs.push([pix.pos, [new_pos[0] as u32, new_pos[1] as u32]]);
        }
        
        pixel_list.sort_by(|a, b| ((a.vel[0] + a.vel[1]) * a.density).partial_cmp(&((b.vel[0] + b.vel[1]) * b.density)).unwrap());

        //for every move, swap pixels
        //if new_grid pos is empty, grab from old grid
        for pair in &pixel_pairs {
            let old_pos = pair[0];
            let new_pos = pair[1];

            let mut pix = self.grid[old_pos[1] as usize][old_pos[0] as usize];
            pix.pos = [new_pos[0] as u32, new_pos[1] as u32];

            if new_grid[new_pos[1] as usize][new_pos[0] as usize].ptype == 0 {
                new_grid[new_pos[1] as usize][new_pos[0] as usize] = pix;
                new_grid[old_pos[1] as usize][old_pos[0] as usize] = self.grid[new_pos[1] as usize][new_pos[0] as usize];
            } else {
                let mut pix2 = new_grid[new_pos[1] as usize][new_pos[0] as usize];
                pix2.pos = [old_pos[0] as u32, old_pos[1] as u32];

                new_grid[new_pos[1] as usize][new_pos[0] as usize] = pix;
                new_grid[old_pos[1] as usize][old_pos[0] as usize] = pix2;
            }
        }

        self.grid = new_grid;
    }

    pub fn print(&self, _verbose: bool) {
        let mut stdout = stdout();
        stdout.queue(cursor::SavePosition).unwrap();
        for y in 0..self.SIZE[1] {
            print!("{:04}|", y);
            for x in 0..self.SIZE[0] {
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
        let x = self.mouse_pos[0];
        let y = self.mouse_pos[1];
        for x in 0..self.SIZE[0] {
            for y in y..self.SIZE[1] {
                if self.grid[y as usize][x as usize].density <= 0.3 {
                    self.grid[y as usize][x as usize] = Pixel::spawn(typ.clone(), [x, y]);
                }
            }
        }
    }

    //pixel radius iterator
    pub fn radius_iter(&mut self, r: u32) -> Vec<[u32; 2]> {
        radius(self.mouse_pos, r, self.SIZE)
    }

    //place pixel (air) at mouse position r=radius
    pub fn erase(&mut self, r: u32, typ: String ) {
        let x = self.mouse_pos[0];
        let y = self.mouse_pos[1];
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

pub fn radius(pos: [u32; 2], r: u32, SIZE: [u32; 2]) -> Vec<[u32; 2]> {
    let x = pos[0];
    let y = pos[1];
    let mut vec = Vec::new();
    for i in 0..r {
        for j in 0..r {
            if i*i + j*j <= r*r {
                if 0 <= x as i32 + (i as i32) && x as i32 + (i as i32) < SIZE[0] as i32 && 0 <= y as i32 + (j as i32) && y as i32 + (j as i32) < SIZE[1] as i32 {
                    vec.push([x + i, y + j]);
                } 
                if 0 <= x as i32 - (i as i32) && x as i32 - (i as i32) < SIZE[0] as i32 && 0 <= y as i32 + (j as i32) && y as i32 + (j as i32) < SIZE[1] as i32 {
                    vec.push([x - i, y + j]);
                }
                if 0 <= x as i32 + (i as i32) && x as i32 + (i as i32) < SIZE[0] as i32 && 0 <= y as i32 - (j as i32) && y as i32 - (j as i32) < SIZE[1] as i32 {
                    vec.push([x + i, y - j]);
                }
                if 0 <= x as i32 - (i as i32) && x as i32 - (i as i32) < SIZE[0] as i32 && 0 <= y as i32 - (j as i32) && y as i32 - (j as i32) < SIZE[1] as i32 {
                    vec.push([x - i, y - j]);
                }
            }
        }
    }
    vec
}

pub fn wrapped_coord(pos: [i32; 2], edge_mode: bool, SIZE: [u32; 2]) -> [u32; 2] {
    let mut bx = SIZE[0] as i32 - 1;
    let mut by = SIZE[1] as i32 - 1;
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