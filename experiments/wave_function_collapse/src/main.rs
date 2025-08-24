use image;

fn main() {

    let imgx = 800;
    let imgy = 800;

    let initial_color = image::Rgb([255, 255, 255]);
    let mut imgbuf = image::RgbImage::from_pixel(imgx, imgy, initial_color);

    let step = 100;
    let x_steps = imgx / step;
    let y_steps = imgy / step;
    println!("x_steps {} y_steps {}", x_steps, y_steps);
    for x in 0..x_steps {
        for y in 0..y_steps {
            let subimage = image::RgbImage::from_pixel(imgx, imgy, image::Rgb([255 / (x + 1) as u8, 255 / (y + 1) as u8, 0]));
            image::imageops::replace(&mut imgbuf, &subimage, (x * step) as i64, (y * step) as i64);
        }
    }

    imgbuf.save("result.png").unwrap();
}
