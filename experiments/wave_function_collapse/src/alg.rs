use image;

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
    println!("current_x {} current_y {}", current_x, current_y);

    // TODO: This feels awkward and similar to a Python generator
    // Is there a way to do something similar in Rust?
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
