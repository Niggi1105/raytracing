use crate::math::{self, Point};
use crate::math::Vector3;
use crate::shapes;
use crate::shapes::Sphere;

use image;
use image::Rgb;

#[derive(Debug, Clone)]
pub struct Camera{
    pos: Point,
    normal: Vector3,
    focal_length: f32,
    background: Rgb<u8>,
}

#[derive(Debug, Clone)]
pub struct Raytracer{
    image_size: (u32, u32),
    output_name: String,
    scene: Scene,
}

#[derive(Debug, Clone)]
pub struct Scene{
    objects: Vec<Sphere>,
    camera: Camera
}

impl Camera{
    pub fn new(pos: Point, normal: Vector3, background: Rgb<u8>) -> Self {
        Self { pos, normal, focal_length: normal.length(), background }
    }
}

impl Scene{
    pub fn new(camera: Camera, objects: Vec<Sphere>) -> Self{
        Self { objects, camera }
    }
}

impl Raytracer{
    pub fn new( scene: Scene, image_size: (u32, u32), output_name: String) -> Self {
        Self { scene, image_size, output_name }
    }

    pub fn color_pixel(&self, x: u32, y: u32) -> Rgb<u8> {
        Vector3::ab_from_points(&self.scene.camera.pos, )        
    }


}


