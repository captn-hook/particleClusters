use piston_window::*;
//use rand::Rng;

mod pixel;
//use pixel::*;

mod simulate;
use simulate::*;

mod draw;
use draw::*;

mod elements;

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
            //prepare cursor arguments, 
            //cursor is drawn on top of everything else
            let mut case = 0;
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

            new_frame(&mut sim.window, &event, &sim.grid, sim.SCALE, case, pos, sim.SIZE);
        }

        //track mouse position
        if let Some(pos) = event.mouse_cursor_args() {
            //integer and scaled mouse position
            let x = pos[0] as u32 / sim.SCALE;
            let y = pos[1] as u32 / sim.SCALE;
            //limit mouse position to grid
            if 0 <= x && x < sim.SIZE[0] && 0 <= y && y < sim.SIZE[1] {
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


        //update simulation
        if let Some(args) = event.update_args() {
            let verbose = false;
            if verbose {
                println!("PRE UPDATE ++++++++++++++++++++++++++++++++++++++++++++++++SCALE: {}, SIZE: {}x{}", sim.SCALE, sim.SIZE[0], sim.SIZE[1]);
                sim.print(verbose);
            }
            sim.update(verbose);
            //wln!("POST UPDATE ++++++++++++++++++++++++++++++++++++++++++++++++SCALE: {}, SIZE: {}x{}", sim.scale, sim.size[0], sim.size[1]);
            //sim.print(verbose);
        }
    }
}