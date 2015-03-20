#![allow(unstable)]
#![feature(box_syntax)]

use std::io::prelude::*;
use std::fs::File;
use std::ops::{Add, Sub, Mul};
use std::num::Float;
use std::default::Default;
use std::rand::random;
use std::sync::TaskPool;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};


#[derive(Debug, Copy, Clone, Default)]
struct Vector {
    x: f64,
    y: f64,
    z: f64
}

#[derive(Debug, Copy, Clone, Default)]
struct Ray {
    o: Vector,
    d: Vector
}

#[derive(Debug, Clone, Default)]
struct Sphere {
    radius: f64,
    position: Vector,
    emission: Vector,
    color: Vector,
}

#[derive(Debug, Default, Clone)]
struct Camera {
    eye: Ray, // origin and direction of cam
    // Field of view:
    right: Vector, // right vector
    up: Vector, // up vector
}

trait Shape {
    fn intersect(self, r: Ray) -> f64;
}

trait ShapeRef {
    fn color(self, r: &Ray, t: f64) -> Vector;
}


impl Shape for Sphere {
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

fn get_ray(cam: &Camera, a: usize, b: usize) -> Ray {
    
    let w = cam.eye.d.norm().smul(-1.0);
    let u = cam.up.cross(w).norm();
    let v = w.cross(u);

    let u0 = -1.0;
    let v0 = -1.0;
    let u1 = 1.0;
    let v1 = 1.0;
    let d = 2.0;

    let across = u.smul(u1-u0);
    let up = v.smul(v1-v0);
    let an = (a as f64) / HEIGHT as f64;
    let bn = (b as f64) / WIDTH as f64;

    let corner = &(&(&cam.eye.o + &u.smul(u0)) + &v.smul(v0)) - &w.smul(d);
    let target = &( &corner + &across.smul(an)) + &up.smul(bn);
    Ray{o: cam.eye.o, d: (&target-&cam.eye.o).norm()}
}

fn get_light(ray: Ray, depth: usize) -> Vector{ 
    let mut t: f64 = 0.0;
    let mut id: usize = 0;
    if intersect(ray, &mut t, &mut id) {
        if depth > 5 {
            return SPHERES[id].emission;
        }

        let r1: f64 = 2.0 * PI* std::rand::random();
        let r2: f64 = std::rand::random();
        let r2s: f64 = r2.sqrt();


        // Hitpoint
        let x: Vector = &ray.o + &ray.d.smul(t);
        let n: Vector = (&x - &SPHERES[id].position).norm();
        let nl = if n.dot(&ray.d) < 0.0 { n } else { n.smul(-1.0) };
        
        let w = nl;
        let u = if w.x.abs() > 0.1 { Vector{x: 0.0, y: 1.0, z: 0.0} } else { Vector{x: 1.0, y: 0.0, z: 0.0 } }.cross(w).norm();
        let v = w.cross(u);

        let d = (&(&u.smul( r1.cos()*r2s )  + &v.smul(r1.sin()*r2s)) + &w.smul((1.0-r2).sqrt())).norm();
        return &SPHERES[id].emission + &(&SPHERES[id].color * &get_light(Ray{o: x, d: d}, depth+1));
    }

    return Default::default();
}

static SPHERES: [Sphere; 9] = [
    Sphere{radius:1e5 as f64,  position: Vector{ x: (1e5 as f64 + 1.0) as f64, y: 40.8 as f64, z: 81.6}, emission: Vector{x: 0.0, y: 0.0, z: 0.0 }, color: Vector{x: 0.75,y: 0.25,z: 0.25}}, // Left 
    Sphere{radius:1e5 as f64,  position: Vector{ x:  -1e5 as f64 + 99.0,y: 40.8 as f64, z: 81.6}, emission: Vector{x: 0.0, y: 0.0, z: 0.0 }, color: Vector{x: 0.25,y: 0.25,z: 0.75}}, // Rght 
    Sphere{radius:1e5 as f64,  position: Vector{ x: 50 as f64, y: 40.8 as f64, z: 1e5 as f64}, emission: Vector{x: 0.0, y: 0.0, z: 0.0 }, color: Vector{x: 0.75,y: 0.75,z: 0.75}}, // Back 
    Sphere{radius:1e5 as f64,  position: Vector{ x: 50 as f64, y: 40.8 as f64, z: -1e5+600 as f64}, emission: Vector{x: 0.0, y: 0.0, z: 0.0 }, color: Vector{x: 1.0, y: 1.0, z: 1.0 }}, // Frnt 
    Sphere{radius:1e5 as f64,  position: Vector{ x: 50 as f64, y:  1e5 as f64, z: 81.6}, emission: Vector{x: 0.0, y: 0.0, z: 0.0 }, color: Vector{x: 0.75,y: 0.75,z: 0.75}}, // Botm 
    Sphere{radius:1e5 as f64,  position: Vector{ x: 50 as f64, y: -1e5+81.6 as f64,z: 81.6}, emission: Vector{x: 0.0, y: 0.0, z: 0.0 }, color: Vector{x: 0.75,y: 0.75,z: 0.75}}, // Top 
    Sphere{radius:16.5, position: Vector{ x: 27.0, y: 16.5 as f64, z: 47.0}, emission: Vector{x: 0.0, y: 0.0, z: 0.0 }, color: Vector{x: 0.999, y: 0.999, z: 0.999}}, // Mirr 
    Sphere{radius:16.5, position: Vector{ x: 73.0, y: 16.5 as f64, z: 78.0}, emission: Vector{x: 0.0, y: 0.0, z: 0.0 }, color: Vector{x: 0.999, y: 0.999, z: 0.999}}, // Glas 
    Sphere{radius:600 as f64,  position: Vector{ x: 50 as f64, y: 681.6-0.27 as f64, z: 81.6}, emission: Vector{x: 12.0, y: 12.0, z: 12.0}, color: Vector{x: 1.0, y: 1.0, z: 1.0}}, //Lite 
];

const WIDTH: usize = 1024;
const HEIGHT: usize = 768;
static PI: f64 = 3.14159265358979323846264338327950288_f64;

fn main() {
    
    let mut cam: Camera = Default::default();
    cam.eye.o = Vector {x: 50.0, y: 52.0, z: 295.6};
    cam.eye.d = Vector {x: 0.0, y: -0.042612, z: -1.0};
    cam.up = Vector{x: 1.0, y: 0.0, z: 0.0};

    let pool = TaskPool::new(std::os::num_cpus());
    let (tx, rx):  (Sender<(usize, usize, Vector)>, Receiver<(usize, usize, Vector)>) = channel();

    let samples: usize = 5000;
    let mut output =  box [[Vector{x: 0.0, y: 0.0, z: 0.0}; WIDTH]; HEIGHT];

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let tx = tx.clone();
            let cam = cam.clone();
            pool.execute(move|| {
                let mut r: Vector = Default::default();
                for _ in range(0, samples) {
                    let ray: Ray = get_ray(&cam, i, j);
                    r = &r + &get_light(ray, 0).smul(1.0/samples as f64);
                }
                tx.send((i, j, Vector{ x: clamp(r.x), y: clamp(r.y), z: clamp(r.z) })).unwrap();
            });
        }
    }

    for p in 0..WIDTH*HEIGHT-1 {
        print!("\rRaytracing... ({:.0}%)", (p as f64) / ((WIDTH*HEIGHT) as f64) * 100.0);
        let (i, j, color) = rx.recv().unwrap(); 
        output[i][j] = color;
    }
    
    println!("Writing Image...");
    let mut f = File::create("image.ppm").unwrap();
    f.write_all( format!("P3\n{} {}\n{}\n", WIDTH, HEIGHT, 255).as_bytes() ).ok();
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            f.write_all( format!("{} {} {} ", to_int(output[i][j].x), to_int(output[i][j].y), to_int(output[i][j].z)).as_bytes() ).ok();
        }
    }}