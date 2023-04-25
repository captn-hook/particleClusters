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
    pub size: [u32; 2],
    pub scale: u32,
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

        let mut elements = ElementList::new();

        const scale: u32 = 120;
        const size: [u32; 2] = [5; 2];
        
        let mut gravity: f64 = 0.1;
        let mut friction: f64 = 0.99;        
        let mut mouse_pos = [0, 0];
        let mut edge_mode: bool = false;
        
        let mut grid = [[Pixel::default(); 5]; 5];
        
        //SET pos in pixels to index in grid
        for y in 0..size[1] {
            for x in 0..size[0] {
                grid[y as usize][x as usize] = Pixel::spawn("air".to_string(), [x, y]);

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
            elements,
        }
    }

    pub fn update(&mut self, verbose: bool) {
        let mut new_grid = [[Pixel::default(); 5]; 5];
        //get list of pixels ordered by and velocity
        let mut pixel_list = Vec::new();

        for y in 0..self.size[1] {
            for x in 0..self.size[0] {
                self.grid[y as usize][x as usize].unblock();
                self.grid[y as usize][x as usize].pos = [x, y];

                pixel_list.push(self.grid[y as usize][x as usize]);
            }
        }

        pixel_list.sort_by(|a, b| ((a.vel[0] + a.vel[1]) * a.density).partial_cmp(&((b.vel[0] + b.vel[1]) * b.density)).unwrap());
        //update each pixel and its swappee into new grid, set blocked, update pos
        for mut pix in &mut pixel_list {
            if pix.is_blocked() {
                continue;
            }
            let mut new_pos = pix.phys_step(self.friction, self.gravity, self.edge_mode);
            let mut pos = pix.pos;
            if pos == new_pos {
                //no swap
                pix.block();
                new_grid[pos[1] as usize][pos[0] as usize] = *pix;
            } else {
                //get one pixel in new pos dir from old pos
                if new_pos[0] > pos[0] {
                    pos[0] += 1;
                } else if new_pos[0] < pos[0] {
                    pos[0] -= 1;
                }
                if new_pos[1] > pos[1] {
                    pos[1] += 1;
                } else if new_pos[1] < pos[1] {
                    pos[1] -= 1;
                }

                new_pos = wrapped_coord(new_pos, self.edge_mode, self.size);
                pos = wrapped_coord(pos, self.edge_mode, self.size);              

                //get swappee

                let mut swappee = self.grid[new_pos[1] as usize][new_pos[0] as usize];
                
                swappee.block();
                pix.block();

                println!("PIX: X {}, Y {}, DENSITY {}", pix.pos[0], pix.pos[1], pix.density);
                println!("SWAPPEE: X {}, Y {}, DENSITY {}", swappee.pos[0], swappee.pos[1], swappee.density);

                swappee.pos = pix.pos;
                println!("swappee pos: {:?}", swappee.pos);
                pix.pos = new_pos;

                new_grid[pix.pos[0] as usize][pix.pos[1] as usize] = *pix;
                new_grid[swappee.pos[0] as usize][swappee.pos[1] as usize] = swappee;

                println!("PIX2: X {}, Y {}, DENSITY {}", new_grid[new_pos[1] as usize][new_pos[0] as usize].pos[0], new_grid[new_pos[1] as usize][new_pos[0] as usize].pos[1], new_grid[new_pos[1] as usize][new_pos[0] as usize].density);
                println!("SWAPPEE2: X {}, Y {}, DENSITY {}", new_grid[pos[1] as usize][pos[0] as usize].pos[0], new_grid[pos[1] as usize][pos[0] as usize].pos[1], new_grid[pos[1] as usize][pos[0] as usize].density);
            }
        }
        self.grid = new_grid;
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
        let x = self.mouse_pos[0];
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

pub fn wrapped_coord(pos: [u32; 2], edge_mode: bool, size: [u32; 2]) -> [u32; 2] {
    let mut x = pos[0];
    let mut y = pos[1];
    if edge_mode {
        //go to other side of grid
        if x > size[0] - 1 {
            x = 0;
        } else if x < 0 {
            x = size[0] - 1;
        }
        if y > size[1] - 1 {
            y = 0;
        } else if y < 0 {
            y = size[1] - 1;
        }
    } else {
        //clamp to edge
        if x > size[0] - 1 {
            x = size[0] - 1;
        } else if x < 0 {
            x = 0;
        }
        if y > size[1] - 1 {
            y = size[1] - 1;
        } else if y < 0 {
            y = 0;
        }
    }
    [x as u32, y as u32]
}