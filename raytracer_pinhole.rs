#![allow(unstable)]
#![feature(box_syntax)]

use std::io::{BufferedWriter, File};
use std::ops::{Add, Sub, Mul};
use std::num::Float;
use std::default::Default;

#[derive(Show, Copy, Clone, Default)]
struct Vector {
    x: f64,
    y: f64,
    z: f64
}

#[derive(Show, Copy, Clone, Default)]
struct Ray {
    o: Vector,
    d: Vector
}

#[derive(Show, Clone, Default)]
struct Sphere {
    radius: f64,
    position: Vector,
    color: Vector,
}

#[derive(Show, Default)]
struct Camera {
    eye: Ray, // origin and direction of cam
    // Field of view:
    right: Vector, // right vector
    up: Vector, // up vector
}

trait Intersect {
    fn intersect(self, r: Ray) -> f64;
}


impl Intersect for Sphere {
    fn intersect(self, r: Ray) -> f64 {
        // Solve t^2*d.d + 2*t*(o-p).d + (o-p).(o-p)-R^2 = 0
        let eps = 1e-4;
        let op = &self.position - &r.o;
        let b = op.dot(&r.d);
        let mut det = b * b - op.dot(&op) + self.radius * self.radius;

        if det < 0.0 {
            return 0.0;
        } else {
            det = det.sqrt();
        }

        if (b - det) > eps {
            return b-det;
        }

        if (b + det) > eps {
            return b+det;
        }

        return 0.0;
    }
}

impl<'a> Add for &'a Vector {
    type Output = Vector;

    fn add(self, other: &'a Vector) -> Vector {
        Vector {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl<'a> Sub for &'a Vector {
    type Output = Vector;

    fn sub(self, other: &'a Vector) -> Vector {
        Vector {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl<'a> Mul for &'a Vector {
    type Output = Vector;

    fn mul(self, other: &'a Vector) -> Vector {
        Vector {x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
    }
}

trait VectorOps {
    fn smul(self, rhs: f64) -> Vector;
    fn norm(self) -> Vector;
    fn cross(self, rhs: Vector) -> Vector;
    fn dot(&self, rhs: &Vector) -> f64;
}

impl VectorOps for Vector {

    fn smul(self, other: f64) -> Vector {
        Vector {x: self.x * other, y: self.y * other, z: self.z * other}
    }

    fn norm(self) -> Vector {
        let normalize = 1.0 / (self.x * self.x + self.y * self.y + self.z * self.z).sqrt() ;
        self.smul( normalize )
    }

    fn cross(self, b: Vector) -> Vector {
        Vector{x: self.y * b.z - self.z * b.y, y: self.z * b.x - self.x * b.z, z: self.x * b.y - self.y * b.x}
    }

    fn dot(&self, other: &Vector) -> f64 {
        (*self).x * (*other).x + (*self).y * (*other).y + (*self).z * (*other).z
    }
}

fn clamp(x: f64) -> f64
{
    if x < 0.0 { 
        return 0.0;
    }
    if x > 1.0 {
        return 1.0;
    }

    x
}

fn to_int(x: f64) -> i64
{
    (clamp(x).powf(1.0 / 2.2) * 255.0 + 0.5) as i64
}

fn intersect(r: Ray, t: &mut f64, id: &mut usize) -> bool
{
    let inf = 10e20f64;
    *t = inf;
    for (i, sphere) in SPHERES.iter().enumerate() {
        let d: f64 = sphere.clone().intersect(r.clone());
        if d != 0.0 && d < *t {
            *t = d;
            *id = i;
        }

    }
    return *t < inf;

}

static SPHERES: [Sphere; 1] = [
    Sphere{ radius: 1.41,  position: Vector{ x:0.0, y: 0.0, z: -1.0}, color: Vector{x: 0.25, y: 0.50, z: 0.75} },
];

fn get_ray(cam: &Camera, a: usize, b: usize) -> Ray {
    
    let w = cam.eye.d.norm().smul(-1.0);
    let u = cam.up.cross(w).norm();
    let v = w.cross(u);

    let u0 = -2.0;
    let v0 = -2.0;
    let u1 = 2.0;
    let v1 = 2.0;
    let d = 2.0;

    let across = u.smul(u1-u0);
    let up = v.smul(v1-v0);
    let an = (a as f64) / HEIGHT as f64;
    let bn = (b as f64) / WIDTH as f64;

    let corner = &(&(&cam.eye.o + &u.smul(u0)) + &v.smul(v0)) - &w.smul(d);
    let target = &( &corner + &across.smul(an)) + &up.smul(bn);
    Ray{o: cam.eye.o, d: (&target-&cam.eye.o).norm()}
}


const WIDTH: usize = 900;
const HEIGHT: usize = 900;

fn main() {
    println!("Raytracing...");

    let mut cam: Camera = Default::default();
    cam.eye.o = Vector {x: 0.0, y: 0.0, z: 2.0};
    cam.eye.d = Vector {x: 0.0, y: 0.0, z: -2.0};
    cam.up = Vector{x: 0.0, y: 1.0, z: 0.0};

    let mut output = box [[Vector{x: 0.0, y: 0.0, z: 0.0}; WIDTH]; HEIGHT];
    for i in range(0, HEIGHT) {
        for j in range(0, WIDTH) {
            //let origin: Vector = Vector { x: i as f64, y: j as f64, z: 0.0};
            //let direction: Vector = Vector { x: 0.0, y: 0.0, z: -1.0};
            let ray: Ray = get_ray(&cam, i, j);
            let mut t: f64 = 0.0;
            let mut id: usize = 0;
            if intersect(ray, &mut t, &mut id) {
                output[i][j] = SPHERES[id].color;
            }
            else {
                output[i][j].x = 0.5;
                output[i][j].y = 0.5;
                output[i][j].z = 0.5;
            }
        }
    }

    println!("Writing Image...");
    let file = File::create(&Path::new("image.ppm"));
    let mut writer = BufferedWriter::new(file);

    writer.write(format!("P3\n{} {}\n{}\n", WIDTH, HEIGHT, 255).as_bytes()).ok();
    for i in range(0, HEIGHT) {
        for j in range(0, WIDTH) {
            writer.write(format!("{} {} {} ", to_int(output[i][j].x), to_int(output[i][j].y), to_int(output[i][j].z)).as_bytes()).ok();
        }
    }
}