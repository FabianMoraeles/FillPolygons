use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

pub fn line(
    framebuffer: &mut Framebuffer,
    start: Vector2,
    end: Vector2,
) {
    let mut x0 = start.x as i32;
    let mut y0 = start.y as i32;
    let x1 = end.x as i32;
    let y1 = end.y as i32;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        framebuffer.put_pixel(x0, y0);
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

pub fn draw_polygon(
    framebuffer: &mut Framebuffer,
    points: &[Vector2],
) {
    if points.len() < 2 {
        return;
    }

    for i in 0..points.len() {
        let start = points[i];
        let end = points[(i + 1) % points.len()]; 
        line(framebuffer, start, end);
    }
}

pub fn fill_polygon(
    framebuffer: &mut Framebuffer,
    points: &[Vector2],
) {
    if points.len() < 3 {
        return;
    }

    let min_y = points.iter().map(|p| p.y).fold(f32::INFINITY, f32::min).floor() as i32;
    let max_y = points.iter().map(|p| p.y).fold(f32::NEG_INFINITY, f32::max).ceil() as i32;

    for y in min_y..=max_y {
        let mut intersections = vec![];

      
        for i in 0..points.len() {
            let p1 = points[i];
            let p2 = points[(i + 1) % points.len()];

            let (y1, y2) = (p1.y, p2.y);
            let (x1, x2) = (p1.x, p2.x);

            if (y1 <= y as f32 && y2 > y as f32) || (y2 <= y as f32 && y1 > y as f32) {
              
                let t = (y as f32 - y1) / (y2 - y1);
                let x = x1 + t * (x2 - x1);
                intersections.push(x as i32);
            }
        }


        intersections.sort();

 
        for pair in intersections.chunks(2) {
            if pair.len() == 2 {
                let (x_start, x_end) = (pair[0], pair[1]);
                for x in x_start..=x_end {
                    framebuffer.put_pixel(x, y);
                }
            }
        }
    }
}


