use vector::Vector;
use ray::Ray;
use rand;

pub struct Camera {
    pub position: Vector,
    pub direction: Vector,
    pub up: Vector,
    pub right: Vector,
    pub screen_distance: f64,
    pub screen_height: f64,
    pub screen_width: f64,
    pub image_height: u32,
    pub image_width: u32,
    pub screen_center: Vector,
    pub super_sampling_n: u32,
    //    pub aperture_size : f64,
    //    pub dof_rays : u32,

    sub_pixel_width: f64,
    sub_pixel_height: f64,
}

impl Camera {
    pub fn new(
        position: Vector,
        look_at: Vector,
        up: Vector,
        screen_distance: f64,
        screen_width: f64,
        image_height: u32,
        image_width: u32,
        super_sampling_n: u32,
    ) -> Camera {
        let direction = position.direction_to(&look_at);
        let right = *(up ^ direction).normalize();
        let up_corrected = *(right ^ direction).normalize();
        let screen_height = screen_width * (image_height as f64 / image_width as f64);
        let pixel_width = screen_width / image_width as f64;
        let pixel_height = screen_height / image_height as f64;
        let sub_pixel_width = pixel_width / super_sampling_n as f64;
        let sub_pixel_height = pixel_height / super_sampling_n as f64;
        Camera {
            position,
            direction,
            up: up_corrected,
            right,
            screen_distance,
            screen_height,
            screen_width,
            image_height,
            image_width,
            screen_center: position + direction * screen_distance,
            super_sampling_n,
            sub_pixel_width,
            sub_pixel_height,
        }
    }

    pub fn construct_rays_through_pixel(&self, x: u32, y: u32) -> SubPixelRayIterator {
        SubPixelRayIterator {
            camera: self,
            x,
            y,
            i: 0,
            j: 0,
        }
    }

    fn get_rand(&self) -> f64 {
        if self.super_sampling_n == 1 {
            0.5
        } else {
            rand::random()
        }
    }
}

pub struct SubPixelRayIterator<'a> {
    camera: &'a Camera,
    x: u32,
    y: u32,
    i: u32,
    j: u32,
}

impl<'a> Iterator for SubPixelRayIterator<'a> {
    type Item = Ray;

    fn next(&mut self) -> Option<Self::Item> {
        if self.j >= self.camera.super_sampling_n {
            self.i += 1;
            self.j = 0;
        }
        if self.i >= self.camera.super_sampling_n {
            return None;
        }

        let sub_pixel_x = self.camera.super_sampling_n * self.x + self.i;
        let sub_pixel_y = self.camera.super_sampling_n * self.y + self.j;
        let x_offset = self.camera.sub_pixel_width * (sub_pixel_x as f64 + self.camera.get_rand() - 0.5) - (self.camera.screen_width as f64 / 2.0);
        let y_offset = self.camera.sub_pixel_height * (sub_pixel_y as f64 + self.camera.get_rand() - 0.5) - (self.camera.screen_height as f64 / 2.0);
        let x_delta = self.camera.right * x_offset;
        let y_delta = self.camera.up * y_offset;
        let sub_cell_point = self.camera.screen_center + x_delta + y_delta;
        let ray = Ray::construct_ray(self.camera.position, sub_cell_point);
        self.j += 1;
        Some(ray)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.camera.super_sampling_n * self.camera.super_sampling_n) as usize;
        (len, Some(len))
    }
}

impl<'a> ExactSizeIterator for SubPixelRayIterator<'a> {}
