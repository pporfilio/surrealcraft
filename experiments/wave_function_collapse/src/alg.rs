use image;
use cgmath::Vector2;


pub struct DemoState {
    pub img: image::RgbaImage,
    step: u32,
    current_x: u32,
    current_y: u32,
}

impl DemoState {
    pub fn new(img: image::RgbaImage, step: u32) -> DemoState {
        Self {
            img,
            step,
            current_x: 0,
            current_y: 0,
        }
    }
}

pub fn step_demo_image(state: &mut DemoState) {
    let img_x = state.img.width();
    let img_y = state.img.height();
    let x_steps = img_x / state.step;
    let y_steps = img_y / state.step;
    //println!("current_x {} current_y {}", current_x, current_y);

    // This is awkward and in Python the code could be cleaner with
    // a generator.
    // Except that this also mixes internal and external state, because
    // I want to update the image rather than make a new one at each step.
    // Rust doesn't have finalized coroutines/generators yet, so I'm just
    // experimenting with my own approaches.
    if state.current_x >= x_steps {
        println!("Image filled, no more updates to do!");
        return;
    } else {
        let subimage = image::RgbaImage::from_pixel(
            state.step,
            state.step,
            image::Rgba([
                255 / (state.current_x + 1) as u8,
                255 / (state.current_y + 1) as u8,
                0,
                255,
            ]),
        );
        image::imageops::replace(
            &mut state.img,
            &subimage,
            (state.current_x * state.step) as i64,
            (state.current_y * state.step) as i64,
        );
    }

    state.current_y += 1;
    if state.current_y >= y_steps {
        state.current_y = 0;
        state.current_x += 1;
    }
}

#[allow(unused)]
pub fn fill_demo_image(img: &mut image::RgbaImage) {
    let imgx = img.width();
    let imgy = img.height();
    let step = 100;
    let x_steps = imgx / step;
    let y_steps = imgy / step;
    println!("x_steps {} y_steps {}", x_steps, y_steps);
    for x in 0..x_steps {
        for y in 0..y_steps {
            let subimage = image::RgbaImage::from_pixel(
                step,
                step,
                image::Rgba([255 / (x + 1) as u8, 255 / (y + 1) as u8, 0, 255]),
            );
            image::imageops::replace(img, &subimage, (x * step) as i64, (y * step) as i64);
        }
    }
}


pub struct WFCState {
    pub img: image::RgbaImage,
}

impl WFCState {
    pub fn new(img: image::RgbaImage) -> Self {
        Self {
            img,
        }
    }

    pub fn next(&mut self) -> &image::RgbaImage {
        return &self.img;
    }
}

    // https://stackoverflow.com/questions/16421033/lazy-sequence-generation-in-rust
pub struct Range2D {
    x_start: i32,
    x_current: i32,
    x_end: i32,
    y_start: i32,
    y_current: i32,
    y_end: i32,
}

impl Range2D {
    pub fn new(x_start: i32, x_end: i32, y_start: i32, y_end: i32) -> Self {
        let x_current = x_start;
        let y_current = y_start;
        Self {
            x_start: x_start,
            x_current: x_current,
            x_end: x_end,
            y_start: y_start,
            y_current: y_current,
            y_end: y_end,
        }
    }
}

impl Iterator for Range2D {
    type Item = Vector2<i32>;

    fn next(&mut self) -> Option<Vector2<i32>> {
        if self.x_start >= self.x_end || self.y_start >= self.y_end {
            // Starting condition wasn't valid
            None
        } else if self.x_current >= self.x_end {
            // We've completed the iteration
            None
        } else {
            let result = Vector2::new(self.x_current, self.y_current);
            self.y_current += 1;
            if self.y_current >= self.y_end {
                self.y_current = self.y_start;
                self.x_current += 1;
            }
            return Some(result);
        }
    }
}
