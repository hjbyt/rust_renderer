#[macro_use]
mod utils;
mod vector;
mod sphere;
mod plane;
mod color;
mod material;
mod ray;
mod hit;
mod model_object;
mod camera;
mod color_image;
mod scene;
mod light;

extern crate image;
extern crate rand;

fn main() {
    let scene = scene_from_file_path("scenes/Simple.txt").unwrap(); //TODO
    let color_image = scene.render();
    let image_buffer = color_image.to_image_buffer();
    image_buffer.save("output.png").unwrap();
    println!("Done");
}

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use model_object::ModelObject;
use material::Material;
use camera::Camera;
use sphere::Sphere;
use plane::Plane;
use light::Light;
use scene::Scene;

fn scene_from_file_path(file_path: &str) -> io::Result<Scene> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut camera: Option<Camera> = None;
    let mut settings: Option<(Color, u32, u32, u32)> = None;
    let mut materials: Vec<Material> = Vec::new();
    let mut objects: Vec<Box<ModelObject>> = Vec::new();
    let mut lights: Vec<Light> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut parts = &mut line.split_whitespace();
        let item_type = parts.next().unwrap(); //Note: this can't fail because line is not empty
        match item_type {
            "cam" => {
                camera = Some(Camera::new(
                    parse_vector(parts),
                    parse_vector(parts),
                    parse_vector(parts),
                    parse_f64(parts),
                    parse_f64(parts),
                    500, //TODO
                    500, //TODO
                ));
            }
            "set" => {
                settings = Some((
                    parse_color(parts),
                    parse_u32(parts),
                    parse_u32(parts),
                    parse_u32(parts),
                ));
            }
            "mtl" => {
                materials.push(Material::new(
                    parse_color(parts),
                    parse_color(parts),
                    parse_color(parts),
                    parse_f64(parts),
                    parse_f64(parts),
                ));
            }
            "sph" => {
                objects.push(Box::new(Sphere {
                    center: parse_vector(parts),
                    radius: parse_f64(parts),
                    material: materials[parse_u32(parts) as usize - 1],
                }));
            }
            "pln" => {
                objects.push(Box::new(Plane {
                    normal: parse_vector(parts),
                    offset: parse_f64(parts),
                    material: materials[parse_u32(parts) as usize - 1],
                }));
            }
            "trg" => {
                unimplemented!();
                //objects.push(Box::new(Triangle::new(
                //    parse_vector(parts),
                //    parse_vector(parts),
                //    parse_vector(parts),
                //    parse_uint(parts),
                //)));
            }
            "lgt" => {
                lights.push(Light::new(
                    parse_vector(parts),
                    parse_color(parts),
                    parse_f64(parts),
                    parse_f64(parts),
                    parse_f64(parts),
                ));
            }
            _ => panic!("Unrecognized scene item") //TODO: return error
        }
    }

    let camera = camera.expect("Camera item not found"); //TODO
    let settings = settings.expect("Settings item not found"); //TODO

    Ok(Scene {
        background_color: settings.0,
        objects,
        camera,
        lights,
    })
}

//TODO: fix error handling
fn parse_f64<'a, I: Iterator<Item=&'a str>>(params: &mut I) -> f64 {
    let param = params.next().expect("not enough params");
    param.parse().expect("expected float")
}

fn parse_u32<'a, I: Iterator<Item=&'a str>>(params: &mut I) -> u32 {
    let param = params.next().expect("not enough params");
    param.parse().expect("expected unsigned integer")
}

use vector::Vector;
use color::Color;

fn parse_vector<'a, I: Iterator<Item=&'a str>>(params: &mut I) -> Vector {
    Vector::new(parse_f64(params), parse_f64(params), parse_f64(params))
}

fn parse_color<'a, I: Iterator<Item=&'a str>>(params: &mut I) -> Color {
    Color::new(parse_f64(params), parse_f64(params), parse_f64(params))
}