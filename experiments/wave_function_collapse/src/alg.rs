use cgmath::Vector2;
use image;
use image::{ImageError, ImageReader, SubImage};
use rand::prelude::IteratorRandom;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::{HashMap, HashSet};
use std::thread::current;

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
    pub sample_cell_id_to_info: HashMap<u32, SampleCellInfo>,
    pub output_cell_ids: Vec<i32>,
    pub next_locations: Range2D,
    pub all_ids: HashSet<u32>,
    pub rng: StdRng,
}
#[derive(Clone)]
pub struct SampleCellInfo {
    pub id: u32,
    pub locations: Vec<cgmath::Vector2<u32>>,
    pub adjacencies: HashSet<u32>,
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

        let sample_cell_id_to_info = HashMap::new();

        let output_cell_ids = vec![-1; (output_cells_x * output_cells_y) as usize];

        let next_locations = Range2D::new(0, output_cells_x as i32, 0, output_cells_y as i32);

        let all_ids = HashSet::new();

        let rng = StdRng::seed_from_u64(1234567);

        Self {
            sample_image_rgba,
            generated_image_rgba,
            cell_height,
            cell_width,
            output_cells_x,
            output_cells_y,
            sample_cell_id_to_info,
            output_cell_ids,
            next_locations,
            all_ids,
            rng,
        }
    }

    pub fn sample_col_count(&self) -> u32 {
        return self.sample_image_rgba.width() / self.cell_width;
    }

    pub fn sample_row_count(&self) -> u32 {
        return self.sample_image_rgba.height() / self.cell_height;
    }

    pub fn sample_cell_as_array(&self, x: u32, y: u32) -> Vec<u8> {
        // crop_immutable returns a SubImage that can be converted to an image with `to_image`
        // Constructing with SubImage::new returned something that didn't match the trait bounds of to_image.
        let cell_subimage = image::imageops::crop_imm(
            &self.sample_image_rgba,
            x * self.cell_width,
            y * self.cell_height,
            self.cell_width,
            self.cell_height,
        );
        let cell_image: image::RgbaImage = cell_subimage.to_image();
        return cell_image.into_raw();
    }

    pub fn initialize_adjacency(&mut self) {
        let sample_cols = self.sample_col_count();
        let sample_rows = self.sample_row_count();

        let mut current_cell_id = 0;
        let mut sample_cell_data_to_info = HashMap::new();
        let mut sample_cell_location_to_id = HashMap::new();

        // Since cells can be duplicated, figuring out which cells
        // are the same ahead of time will make it faster to build
        // the adjacency list.
        for x_step in 0..sample_cols {
            for y_step in 0..sample_rows {
                let key = self.sample_cell_as_array(x_step, y_step);

                // Create a new entry only if we don't already have a cell with the same pixel data
                let cell_entry = sample_cell_data_to_info.entry(key).or_insert_with(|| {
                    let new_cell = SampleCellInfo {
                        id: current_cell_id,
                        locations: Vec::new(),
                        adjacencies: HashSet::new(),
                    };
                    self.all_ids.insert(current_cell_id);
                    // Update the cell id inside the closure so that it's only updated when we make a new entry
                    current_cell_id += 1;
                    return new_cell;
                });

                // If cell_entry was a primitive, for example if it was a u32 that we wanted to increment,
                // This would be done as *cell_entry += 1. I guess it returns a reference to primitives but not
                // to structs, or the `.` operator automatically knows if it needs to dereference something?
                // Add the current location to this cell info, for both new and duplicate cell pixel data
                cell_entry
                    .locations
                    .push(cgmath::Vector2::new(x_step, y_step));

                sample_cell_location_to_id
                    .insert(cgmath::Vector2::new(x_step, y_step), cell_entry.id);

                // Doing this for every index
                current_cell_id += 1;
            }
        }

        // into_values consumes the values and means we don't have to do a clone
        // Also ensures we don't try to use it later.
        for value in sample_cell_data_to_info.into_values() {
            self.sample_cell_id_to_info.insert(value.id, value);
        }

        let col_count = self.sample_col_count();
        let row_count = self.sample_row_count();

        println!("location map keys");
        for key in sample_cell_location_to_id.keys() {
            println!("({}, {})", key.x, key.y);
        }

        // For each CellInfo, for each location that cell appears, for each of the location's neighbors,
        // record the neighbor's id in the adjacencies set.
        for value in self.sample_cell_id_to_info.values_mut() {
            println!("Cell id {}", value.id);
            for location in value.locations.clone() {
                println!("Cell location: ({}, {})", location.x, location.y);
                println!("Adjacencies:");
                for adjacent_location in
                    WFCState::adjacent_cell_coordinates(location, col_count, row_count)
                {
                    println!(
                        "Adjacent location: ({}, {})",
                        adjacent_location.x, adjacent_location.y
                    );
                    value
                        .adjacencies
                        .insert(*sample_cell_location_to_id.get(&adjacent_location).unwrap());
                }
            }
        }
    }

    pub fn adjacent_cell_coordinates(
        location: cgmath::Vector2<u32>,
        col_count: u32,
        row_count: u32,
    ) -> Vec<cgmath::Vector2<u32>> {
        let mut result = Vec::new();
        let x = location.x;
        let y = location.y;

        if x > 0 {
            result.push(cgmath::Vector2::new(x - 1, y));
        }
        if y > 0 {
            result.push(cgmath::Vector2::new(x, y - 1));
        }
        if x < col_count - 1 {
            result.push(cgmath::Vector2::new(x + 1, y));
        }
        if y < row_count - 1 {
            result.push(cgmath::Vector2::new(x, y + 1));
        }

        return result;
    }

    pub fn sample_texture_offset(
        &self,
        cell_location: cgmath::Vector2<u32>,
    ) -> cgmath::Vector2<f32> {
        return cgmath::Vector2::new(
            cell_location.x as f32
                * (self.cell_width as f32 / self.sample_image_rgba.width() as f32),
            cell_location.y as f32
                * (self.cell_height as f32 / self.sample_image_rgba.height() as f32),
        );
    }

    pub fn sample_texture_scale(&self) -> cgmath::Vector2<f32> {
        return cgmath::Vector2::new(
            1.0 / self.sample_col_count() as f32,
            1.0 / self.sample_row_count() as f32,
        );
    }

    pub fn step_algorithm(&mut self) {
        // Get the next location to visit
        let Some(current_location_i32) = self.next_locations.next() else {
            return;
        };

        let current_location = Vector2 {
            x: current_location_i32.x as u32,
            y: current_location_i32.y as u32,
        };
        let mut available_ids = self.all_ids.clone();

        // Get the coordinates adjacent to that location
        println!("Checking adjacent coordinates");
        for coordinates in WFCState::adjacent_cell_coordinates(
            current_location,
            self.output_cells_x,
            self.output_cells_y,
        ) {
            // Get the allowed adjacent ids for the id at each adjacent location
            // If the id is -1, all ids are allowed, so can skip this
            // To get the allowed adjacent ids, look up in sample_id_to_info
            print!("coordinates: {}, {}", coordinates.x, coordinates.y);
            let id_at_coordinate = self.output_cell_ids
                [(coordinates.y * self.output_cells_x + coordinates.x) as usize];
            if id_at_coordinate != -1 {
                available_ids = &available_ids
                    & &self
                        .sample_cell_id_to_info
                        .get(&(id_at_coordinate as u32))
                        .unwrap()
                        .adjacencies;
            }
            println!("Remaining id options: {:?}", available_ids);
        }

        // If it's empty, do nothing, for now
        // TODO:
        if available_ids.len() == 0 {
            println!("No viable tiles!");
            return;
        }

        // Otherwise, select an id at random
        let picked_ref = available_ids.iter().choose(&mut self.rng).unwrap();

        // Assign the id to the current location
        self.output_cell_ids
            [(current_location.y * self.output_cells_x + current_location.x) as usize] =
            *picked_ref as i32;

        self.update_generated_image(current_location, *picked_ref);
    }

    pub fn update_generated_image(&mut self, destination_location: Vector2<u32>, source_id: u32) {
        let source_location = self
            .sample_cell_id_to_info
            .get(&source_id)
            .unwrap()
            .locations[0];

        let cell_subimage = image::imageops::crop_imm(
            &self.sample_image_rgba,
            source_location.x * self.cell_width,
            source_location.y * self.cell_height,
            self.cell_width,
            self.cell_height,
        );

        let cell_image: image::RgbaImage = cell_subimage.to_image();

        image::imageops::replace(
            &mut self.generated_image_rgba,
            &cell_image,
            (destination_location.x * self.cell_width) as i64,
            (destination_location.y * self.cell_height) as i64,
        );
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
