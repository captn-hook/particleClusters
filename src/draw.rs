use rand::Rng;
use piston_window::*;

use crate::pixel::*;

use crate::simulate::*;

pub fn rect_pos(pos1: [u32; 2], pos2: [u32; 2]) -> [u32; 4] {
    //get top left and bottom right corners
    //four cases: x1 < x2, x1 > x2, y1 < y2, y1 > y2
    let x = pos1[0] as i32 - pos2[0] as i32;
    let y = pos1[1] as i32 - pos2[1] as i32;
    let mut x1: u32;
    let mut x2: u32;
    let mut y1: u32;
    let mut y2: u32;

    if x < 0 {
        //println!("x < 0");
        x1 = pos1[0];
        x2 = pos2[0];
    } else {
        //println!("x > 0");
        x1 = pos2[0];
        x2 = pos1[0];
    }
    if y < 0 {
        //println!("y < 0");
        y1 = pos1[1];
        y2 = pos2[1];
    } else {
        //println!("y > 0");
        y1 = pos2[1];
        y2 = pos1[1];
    }

    [x1, y1, x2, y2]
}

pub fn screem(pos: [u32; 2], scale: u32) -> [f64; 4] {
    let tx = (pos[0] * scale) as f64;
    let ty = (pos[1] * scale) as f64;
    let by = ty + scale as f64;
    let lx = tx + scale as f64;
    [tx, ty, lx, by]
}

pub fn draw_cursor_outline(pos: [f64; 4], context: Context, graphics: &mut G2d) {

    let tx = pos[0];
    let ty = pos[1];
    let lx = pos[2];
    let by = pos[3];
    
    let line = [tx, ty, lx, ty];
    let line2 = [tx, ty, tx, by];
    let line3 = [tx, by, lx, by];
    let line4 = [lx, ty, lx, by];
    for (i, line) in [line, line2, line3, line4].iter().enumerate() {
        let lined = line::Line::new([0.0, 0.0, 0.0, 1.0], 1.0);
        lined.draw(*line, &context.draw_state, context.transform, graphics);
    }

}

pub fn get_screen_edge(pos: [u32; 4], scale: u32) -> [f64; 4] {
    //get top left and bottom right corners
    //use rect_pos to get the top left and bottom right corners
    let pos2 = rect_pos([pos[0], pos[1]], [pos[2], pos[3]]);
    let topleft = screem([pos2[0], pos2[1]], scale);
    let bottomright = screem([pos2[2], pos2[3]], scale);
    [topleft[0], topleft[1], bottomright[2], bottomright[3]]
}

pub fn draw_outline(pos: [f64; 4], context: Context, graphics: &mut G2d) {

    let tx = pos[0];
    let ty = pos[1];
    let lx = pos[2];
    let by = pos[3];
    
    let line = [tx, ty, lx, ty];
    let line2 = [tx, ty, tx, by];
    let line3 = [tx, by, lx, by];
    let line4 = [lx, ty, lx, by];
    for (i, line) in [line, line2, line3, line4].iter().enumerate() {
        let lined = line::Line::new([0.0, 0.0, 0.0, 1.0], 1.0);
        lined.draw(*line, &context.draw_state, context.transform, graphics);
    }

}

pub fn new_frame(window: &mut PistonWindow, event: &Event, pixels: &[[Pixel; 5]; 5], scale: u32, case: u32, pos: [u32; 4], size: [u32; 2]) {
    window.draw_2d(event, |context, graphics, _| {
        clear([1.0; 4], graphics);
        //pixel.pixel_draw
        pixel_draw(*pixels, context, graphics, scale);
        //draw cursor\
        let mut dpos = [0.0; 4];
        if case == 1 {
            //holding left click, draw from last click to mouse pos
            draw_outline(get_screen_edge(pos, scale), context, graphics);

        } else if case == 2 {
            //radius
            let radius_vec = radius([pos[0], pos[1]], pos[2], size);
            //get all outward facing edges as segments vec<[[f64; 2]; 2]>
            let mut segments: Vec<[[f64; 2]; 2]> = Vec::new();

            //get bounding box
            let mut bb_x = [radius_vec[0][0], radius_vec[0][0]];
            let mut bb_y = [radius_vec[0][1], radius_vec[0][1]];

            for pix in radius_vec.iter() {
                if pix[0] < bb_x[0] {
                    bb_x[0] = pix[0];
                } else if pix[0] > bb_x[1] {
                    bb_x[1] = pix[0];
                } 
                if pix[1] < bb_y[0] {
                    bb_y[0] = pix[1];
                } else if pix[1] > bb_y[1] {
                    bb_y[1] = pix[1];
                }
            }

            //just draw the bounding box for now
            draw_outline(get_screen_edge([bb_x[0], bb_y[0], bb_x[1], bb_y[1]], scale), context, graphics);

        } else if case == 3 {
            //line from edge to edge at y height
            draw_outline([0.0, pos[1] as f64 * scale as f64, size[0] as f64 * scale as f64, pos[1] as f64 * scale as f64 + scale as f64], context, graphics);
            
        } else if case == 4 {
            //draw cursor outline
            draw_cursor_outline(screem([pos[0], pos[1]], scale), context, graphics);
        }
        
    });
}
        


pub fn rand_color_margin(color: [f32; 4], margin: f32) -> [f32; 4] {
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

pub fn rand_color_grey(value: f32, margin: f32) -> [f32; 4] {
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

pub fn rand_color(value: f32, margin: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range((value - margin)..(value + margin))
}