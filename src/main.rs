use piston_window::*;
//use rand::Rng;

mod pixel;
//use pixel::*;

mod simulate;
use simulate::*;

mod draw;
use draw::*;

mod elements;
use std::io::{stdin, stdout, Read, Write};
use std::thread;

fn _pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
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
    let mut alt = false;
    let mut _tab = false;
    
    while let Some(event) = sim.window.next() {

        //draw on render
        if let Some(_args) = event.render_args() {
            //prepare cursor arguments, 
            //cursor is drawn on top of everything else
            let case: u8;
            let mut pos = [sim.mouse_pos[0], sim.mouse_pos[1], last_left_click[0], last_left_click[1]];
                
            if left_click {
                //from last click to mouse pos
                case = 1;

            } else if right_click {
                //radius
                case = 2;

                pos[2] = 5;
                pos[3] = 5;

            } else if space {
                //line across screen
                case = 3;
            } else {
                //1 px cursor
                case = 4;
            }

            new_frame(&mut sim.window, &event, &sim.grid, sim.scale, case, pos, sim.size);
        }

        //track mouse position
        if let Some(pos) = event.mouse_cursor_args() {
            //integer and scaled mouse position
            let x = pos[0] / sim.scale as f64;
            let y = pos[1] / sim.scale as f64;
            //limit mouse position to grid
            if 0.0 <= x && x < sim.size[0] as f64 && 0.0 <= y && y < sim.size[1] as f64 {
                sim.mouse_pos = [x as u32, y as u32];
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
                sim.place_line(last_left_click, "stone".to_string());
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
            if key == Key::LAlt {
                alt = true;
            }
            if key == Key::Tab {
                _tab = true;
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
            if key == Key::LAlt {
                alt = false;
            }
            if key == Key::Tab {
                _tab = false;
                sim.edge_mode = !sim.edge_mode;
            }
        }

        //middle click to to place sand
        if middle_click {
            sim.place_pixel("sand".to_string());
        }
        //right click to erase (air)
        if right_click {
            sim.erase(5, "air".to_string());
        }
        //space to place water
        if space {
            sim.sea("water".to_string());
        }
        if shift {
            sim.place_pixel("brick".to_string());
        }
        if ctrl {
            sim.place_pixel("lava".to_string());
        }
        if alt {
            sim.place_pixel("wood".to_string());
        }


        //update simulation
        if let Some(_args) = event.update_args() {
            let verbose = false;
            if verbose {
                println!("PRE UPDATE ++++++++++++++++++++++++++++++++++++++++++++++++scale: {}, size: {}x{}", sim.scale, sim.size[0], sim.size[1]);
                sim.print(verbose);
            }
                
            let subgrids = sim.update_grids(verbose);
            //multithread the subdate fn
            let mut handles = vec![];
            for i in 0..sim.chunk_div {
                let subgrid = subgrids[i as usize].clone();
                let handle = thread::spawn(move || {
                    let (sg, eg) = subdate(subgrid, i, sim.size, sim.chunk_div, sim.gravity, sim.friction, sim.edge_mode);
                    (sg, eg)
                });
                handles.push(handle);
            }

            let mut subgrids2 = vec![];
            let mut edge_cases2 = vec![];
            for handle in handles {
                let (subgrid, edge_cases) = handle.join().unwrap();
                subgrids2.push(subgrid);
                edge_cases2.push(edge_cases);
            }

            sim.update_whole(subgrids2, edge_cases2)
        }
    }
}