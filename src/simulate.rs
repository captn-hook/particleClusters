use piston_window::*;
use rand::Rng;
use crossterm::{QueueableCommand, cursor, terminal, ExecutableCommand};

use std::{thread, time};
use std::io::{Write, stdout};

use colored::Colorize;g

use crate::pixel::*;
use crate::draw::*;
use crate::elements::*;

pub struct Simulation {
    pub size: [u32; 2],
    pub scale: u32,
    pub window: PistonWindow,
    pub grid: [[Pixel; 50]; 50],
    pub gravity: f64,
    pub friction: f64,
    pub mouse_pos: [u32; 2],
    pub edge_mode: bool,
    pub elements: ElementList,
}

impl Simulation {
    pub fn new() -> Simulation {

        let mut elements = ElementList::new();

        const scale: u32 = 10;
        const size: [u32; 2] = [50; 2];
        
        let mut gravity: f64 = 0.1;
        let mut friction: f64 = 0.99;        
        let mut mouse_pos = [0, 0];
        let mut edge_mode: bool = true;
        
        let mut grid = [[Pixel::default(); 50]; 50];
        
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
        let mut collisions: Vec<(Pixel, Pixel)> = vec![];
        
        for y in 0..self.size[1] {
            for x in 0..self.size[0] {
                let mut pixel = self.grid[y as usize][x as usize];
                let collide = pixel.update(self.grid, self.gravity, self.friction, self.edge_mode, [0, 0]);
                //if collide option isnt none, add to collisions
                if let Some((p1, p2)) = collide {
                    collisions.push((p1, p2));
                }
            }
        }

        for collision in collisions {
            let pixel1 = collision.0;
            let pixel2 = collision.1;
            
            println!("COL: ({}, {}) ({}, {})", pixel1.pos[0], pixel1.pos[1], pixel2.pos[0], pixel2.pos[1]);

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