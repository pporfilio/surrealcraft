use cgmath::Vector2;
use image;
use image::{ImageError, ImageReader, SubImage};
use std::collections::HashMap;

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
    pub sample_image_rgba: image::RgbaImage,
    pub generated_image_rgba: image::RgbaImage,
    pub cell_height: u32,
    pub cell_width: u32,
    pub output_cells_x: u32,
    pub output_cells_y: u32,
}

pub struct SampleCell {
    pub id: u32,
    pub locations: Vec<cgmath::Vector2<u32>>,
}

pub struct HashTest {
    pub count: u32,
}

impl WFCState {
    pub fn new(
        sample_image: image::DynamicImage,
        cell_width: u32,
        cell_height: u32,
        output_cells_x: u32,
        output_cells_y: u32,
    ) -> Self {
        let sample_image_rgba = sample_image.to_rgba8();

        assert!(
            sample_image.width() % cell_width == 0,
            "Sample image width must be divisible by cell_width"
        );
        assert!(
            sample_image.height() % cell_height == 0,
            "Sample image height must be divisible by cell_height"
        );

        let initial_color = image::Rgba([255, 255, 255, 255]);
        let generated_image_rgba = image::RgbaImage::from_pixel(
            output_cells_x * cell_width,
            output_cells_y * cell_height,
            initial_color,
        );

        Self {
            sample_image_rgba,
            generated_image_rgba,
            cell_height,
            cell_width,
            output_cells_x,
            output_cells_y,
        }
    }

    pub fn initialize_adjacency(&self) {
        let sample_cols = self.sample_image_rgba.width() / self.cell_width;
        let sample_rows = self.sample_image_rgba.height() / self.cell_height;

        let mut cell_map = HashMap::new();

        let current_cell_id = 0;

        // Since cells can be duplicated, figuring out which cells
        // are the same ahead of time will make it faster to build
        // the adjacency list.
        for x_step in 0..sample_cols {
            for y_step in 0..sample_rows {
                // crop_immutable returns a SubImage that can be converted to an image with `to_image`
                // Constructing with SubImage::new returned something that didn't match the trait bounds of to_image.
                let cell_subimage = image::imageops::crop_imm(
                    &self.sample_image_rgba,
                    x_step * self.cell_width,
                    y_step * self.cell_height,
                    self.cell_width,
                    self.cell_height,
                );
                let cell_image: image::RgbaImage = cell_subimage.to_image();
                let key: Vec<u8> = cell_image.into_raw();
                let cell_entry = cell_map.entry(key).or_insert(SampleCell {
                    id: current_cell_id,
                    locations: Vec::new(),
                });

                // If cell_entry was a primitive, for example if it was a u32 that we wanted to increment,
                // This would be done as *cell_entry += 1. I guess it returns a reference to primitives but not
                // to structs, or the `.` operator automatically knows if it needs to dereference something?
                cell_entry
                    .locations
                    .push(cgmath::Vector2::new(x_step, y_step));

                current_cell_id += 1;
            }
        }

        // make a map between cell contents and id

        // make a 2d array with each cell's content's id

        // build adjacency list from this array
    }

    pub fn next(&mut self) -> &image::RgbaImage {
        return &self.generated_image_rgba;
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
    /// Creates a new Range2D iterator.
    ///
    /// # Arguments
    /// * `x_start` - Starting x-coordinate (inclusive)
    /// * `x_end` - Ending x-coordinate (exclusive)
    /// * `y_start` - Starting y-coordinate (inclusive)
    /// * `y_end` - Ending y-coordinate (exclusive)
    ///
    /// # Panics
    /// Panics if any start is greater than its corresponding end.
    pub fn new(x_start: i32, x_end: i32, y_start: i32, y_end: i32) -> Self {
        assert!(x_start <= x_end, "x_start must be <= x_end");
        assert!(y_start <= y_end, "y_start must be <= y_end");
        Self {
            x_start,
            x_current: x_start,
            x_end,
            y_start,
            y_current: y_start,
            y_end,
        }
    }
}

impl Iterator for Range2D {
    type Item = Vector2<i32>;

    fn next(&mut self) -> Option<Vector2<i32>> {
        if self.x_current >= self.x_end {
            // We've completed the iteration
            None
        } else {
            // Get the current position
            let result = Vector2::new(self.x_current, self.y_current);

            // Move to the next position after storing the current, so that
            // we are inclusive of the starting position.
            self.y_current += 1;
            if self.y_current >= self.y_end {
                self.y_current = self.y_start;
                self.x_current += 1;
            }

            return Some(result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range2d_iteration() {
        let mut iter = Range2D::new(0, 2, 0, 2);
        assert_eq!(iter.next(), Some(Vector2::new(0, 0)));
        assert_eq!(iter.next(), Some(Vector2::new(0, 1)));
        assert_eq!(iter.next(), Some(Vector2::new(1, 0)));
        assert_eq!(iter.next(), Some(Vector2::new(1, 1)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    #[should_panic(expected = "x_start must be <= x_end")]
    fn test_invalid_range_x() {
        let _ = Range2D::new(2, 1, 0, 1);
    }

    #[test]
    #[should_panic(expected = "y_start must be <= y_end")]
    fn test_invalid_range_y() {
        let _ = Range2D::new(0, 1, 2, 1);
    }
}
