use vector::Vector;

pub struct Camera {
    pub position: Vector,
    pub direction: Vector,
    pub up: Vector,
    pub right: Vector,
    pub screen_distance: f64,
    pub screen_height: f64,
    pub screen_width: f64,
    pub image_height: i64,
    pub image_width: i64,
    pub screen_center: Vector,
//    pub aperature_size : f64,
//    pub dof_rays : i64,
}

impl Camera {
    pub fn new(
        position: Vector,
        look_at: Vector,
        up: Vector,
        screen_distance: f64,
        screen_width: f64,
        image_height: i64,
        image_width: i64
    ) -> Camera {
        let direction = position.direction_to(look_at);
        let right = (up ^ direction).normalize();
        let up_corrected = (right ^ direction).normalize();
        Camera {
            position: position,
            direction: direction,
            up: up,
            right: right,
            screen_distance: screen_distance,
            screen_height: screen_width * (image_height as f64 / image_width as f64),
            screen_width: screen_width,
            image_height: image_height,
            image_width: image_width,
            screen_center: position + direction * screen_distance
        }
    }
}