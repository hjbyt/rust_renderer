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
        Camera {
            position,
            direction,
            up: up_corrected,
            right,
            screen_distance,
            screen_height: screen_width * (image_height as f64 / image_width as f64),
            screen_width,
            image_height,
            image_width,
            screen_center: position + direction * screen_distance,
            super_sampling_n
        }
    }

    pub fn construct_rays_through_pixel(&self, x: u32, y: u32) -> Vec<Ray> {
        let n = self.super_sampling_n;
        let pixel_width = self.screen_width / self.image_width as f64;
        let pixel_height = self.screen_height / self.image_height as f64;
        let sub_pixel_width = pixel_width / n as f64;
        let sub_pixel_height = pixel_height / n as f64;
        //TODO: instead of creating new array, fill array given by arg, or yield rays as an iterator.
        let mut rays = Vec::with_capacity((n * n) as usize);
        for i in 1..n+1 {
            let sub_pixel_x = n * x + i;
            for j in 1..n+1 {
                let sub_pixel_y = n * y + j;
                let x_offset = sub_pixel_width * (sub_pixel_x as f64 + self.get_rand() - 0.5) - (self.screen_width as f64 / 2.0);
                let y_offset = sub_pixel_height * (sub_pixel_y as f64 + self.get_rand() - 0.5) - (self.screen_height as f64 / 2.0);
                let x_delta = self.right * x_offset;
                let y_delta = self.up * y_offset;
                let sub_cell_point = self.screen_center + x_delta + y_delta;
                let ray = Ray::construct_ray(self.position, sub_cell_point);
                rays.push(ray);
            }
        }
        rays
    }

    fn get_rand(&self) -> f64 {
        if self.super_sampling_n == 1 {
            0.5
        } else {
            rand::random()
        }
    }
}

