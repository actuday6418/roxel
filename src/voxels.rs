extern crate image;

type HData = image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>>;
type CData = image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>;

pub struct VoxelData {
    color_data: CData,
    height_data: HData,
    pub width: f32,
    pub height: f32,
}

impl VoxelData {
    pub fn new(color_file: &str, height_file: &str) -> Self {
        let cd = image::open(color_file)
            .unwrap()
            .as_rgb8()
            .expect("Couldn't process image!")
            .clone();
        let hd = image::open(height_file)
            .unwrap()
            .as_luma8()
            .expect("Couldn't process image!")
            .clone();
        let vd = VoxelData {
            width: cd.dimensions().0 as f32,
            height: cd.dimensions().1 as f32,
            color_data: cd,
            height_data: hd,
        };
        let (h_width, h_height) = vd.height_data.dimensions();
        if vd.width != h_width as f32 || vd.height != h_height as f32 {
            panic!("Image files have different dimensions!");
        }
        vd
    }

    pub fn get_height(&self, x: &mut i32, y: &mut i32) -> f32 {
        if *y < 0 || *y >= self.height as i32 {
            *y = y.rem_euclid(self.height as i32);
        }
        if *x < 0 || *x >= self.height as i32 {
            *x = x.rem_euclid(self.height as i32);
        }
        return self.height_data.get_pixel(*x as u32, *y as u32)[0] as f32;
    }

    pub fn get_color(&self, x: &mut i32, y: &mut i32) -> [f32; 3] {
        if *y < 0 || *y >= self.height as i32 {
            *y = y.rem_euclid(self.height as i32);
        }
        if *x < 0 || *x >= self.height as i32 {
            *x = x.rem_euclid(self.height as i32);
        }
        return [
            self.color_data.get_pixel(*x as u32, *y as u32)[0] as f32 / 255f32,
            self.color_data.get_pixel(*x as u32, *y as u32)[1] as f32 / 255f32,
            self.color_data.get_pixel(*x as u32, *y as u32)[2] as f32 / 255f32,
        ];
    }
}
