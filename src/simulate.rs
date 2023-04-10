//use rand::Rng;
use crate::Simulation;
use crate::pixel::*;

pub fn update(sim: &mut Simulation, dt: f64) {
    let mut rng = rand::thread_rng();
    let mut new_grid = sim.grid.clone();
    let mut collisions: Vec<(([u32; 2], [u32; 2]), Vec<Pixel>)> = Vec::new();

    for y in 0..sim.size[1] {
        for x in 0..sim.size[0] {
            let mut pixel = sim.grid[y as usize][x as usize];
            let mut new_pos = pixel.pos;
            let mut new_vel = pixel.vel;

            //Gravity
            new_vel[1] += sim.gravity * pixel.density * pixel.gravity_multiplier * dt;

            //Friction
            new_vel[0] *= pixel.friction_multiplier;
            new_vel[1] *= pixel.friction_multiplier;

            //Move=
            new_pos[0] = (new_pos[0] as f64 + new_vel[0] * dt) as u32;
            new_pos[1] = (new_pos[1] as f64 + new_vel[1] * dt) as u32;

            //Collisions
            let mut steps = sim.line_iter(pixel.pos, new_pos);

            //if edge, bounce, else, swap if higher density
            if new_pos[0] >= sim.size[0] || new_pos[0] < 0 && sim.edge_mode {
                new_vel[0] *= -1.0;
            }
            if new_pos[1] >= sim.size[1] || new_pos[1] < 0 && sim.edge_mode {
                new_vel[1] *= -1.0;
            }
            if !sim.edge_mode {
                let mut dest = [0; 2];
                if new_pos[0] >= sim.size[0] {
                    dest[0] = sim.size[0] - 1;
                    dest[1] = new_pos[1];
                } else if new_pos[0] < 0 {
                    dest[0] = 0;
                    dest[1] = new_pos[1];
                } else if new_pos[1] >= sim.size[1] {
                    dest[0] = new_pos[0];
                    dest[1] = sim.size[1] - 1;
                } else if new_pos[1] < 0 {
                    dest[0] = new_pos[0];
                    dest[1] = 0;
                }

                if sim.grid[dest[1] as usize][dest[0] as usize].density < pixel.density {
                    steps.push(sim.grid[dest[1] as usize][dest[0] as usize]);
                    new_pos = dest
                }
            }

            //add to collisions
            collisions.push(((pixel.pos, new_pos), steps));

        }
    }

    //apply collisions
    collide(sim, collisions);
}

fn collide(sim: &mut Simulation, collisions: Vec<(([u32; 2], [u32; 2]), Vec<Pixel>)>) {
    //collisions: Vec<((old_pos, new_pos), steps)>
    //go through each step and swap pixels if swappee is lower density
    for collision in collisions {
        let start = sim.grid[collision.0.0[1] as usize][collision.0.0[0] as usize];
        let end = collision.0.1;
        let steps = collision.1;
        let mut last = start;
        for step in steps {
            if last.density < step.density {
                let temp = last;
                last = step;
                sim.grid[temp.pos[1] as usize][temp.pos[0] as usize] = temp;
            } else {
                sim.grid[step.pos[1] as usize][step.pos[0] as usize] = last;
                last = step;
            }
        }
    }
}