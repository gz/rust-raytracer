#![feature(box_syntax)]

extern crate opencl;

use std::io::{BufferedWriter, File};
use opencl::mem::CLBuffer;
use opencl::hl::EventList;
use std::fmt;
use std::vec::{Vec};
use std::num::Float;
use opencl::array::*;

fn clamp(x: f32) -> f32
{
    if x < 0.0 { 
        return 0.0;
    }
    if x > 1.0 {
        return 1.0;
    }

    x
}

fn to_int(x: f32) -> i64
{
    (clamp(x).powf(1.0f32 / 2.2f32) * 255.0 + 0.5) as i64
}

const HEIGHT: usize = 768;
const WIDTH: usize = 1024;
const SAMPLES: usize = 0;

fn main()
{
    let ker = include_str!("raytracer.ocl");
    println!("ker {}", ker);

    let (device, ctx, queue) = opencl::util::create_compute_context().unwrap();
    println!("{}", device.name());

    let program = ctx.create_program_from_source(ker);
    match program.build(&device) {
        Ok(_) => (),
        Err(build_log) => {
            println!("Error building program:\n");
            println!("{}", build_log);
            panic!("");
        }
    }

    let kernel = program.create_kernel("vector_add");

    let arr_in_x = Array2D::new(768, 1024, |x, y| { 0.0f32 });
    let arr_x = ctx.create_buffer_from(&arr_in_x, opencl::cl::CL_MEM_READ_WRITE);
    kernel.set_arg(0, &arr_x);

    let arr_in_y = Array2D::new(768, 1024, |x, y| { 0.0f32 });
    let arr_y = ctx.create_buffer_from(&arr_in_y, opencl::cl::CL_MEM_READ_WRITE);
    kernel.set_arg(1, &arr_y);
    
    let arr_in_z = Array2D::new(768, 1024, |x, y| { 0.0f32 });
    let arr_z = ctx.create_buffer_from(&arr_in_z, opencl::cl::CL_MEM_READ_WRITE);
    kernel.set_arg(2, &arr_z);

    /*let mut vec_randoms: Vec<f32> = Vec::new();
    for i in range(0, SAMPLES*2) {
        let a: f32 = std::rand::random();
        assert!(a < 1.0f32);
        assert!(a > 0.0f32);
        vec_randoms.push(a);
    }
    println!("Randoms generated");*/
    //let randoms: CLBuffer<f32> = ctx.create_buffer_from(&vec_randoms, opencl::cl::CL_MEM_READ_ONLY);
    //kernel.set_arg(3, &randoms);

    //let samples = ctx.create_buffer_from(vec![12.0f32], opencl::cl::CL_MEM_READ_WRITE);
    //kernel.set_arg(4, &samples);

    queue.enqueue_async_kernel(&kernel, (HEIGHT, WIDTH), None, ());//.wait();
   
   
    let vec_x: Array2D<(f32)> = queue.get(&arr_x, ());
    let vec_y: Array2D<(f32)> = queue.get(&arr_y, ());
    let vec_z: Array2D<(f32)> = queue.get(&arr_z, ());
    //let samples: Vec<(f32)> = queue.get(&samples, ());
    //println!("{:?}", samples[0]);



    println!("\nWriting Image...");
    let file = File::create(&Path::new("image.ppm"));
    let mut writer = BufferedWriter::new(file);

    writer.write(format!("P3\n{} {}\n{}\n", WIDTH, HEIGHT, 255).as_bytes()).ok();
    for i in range(0, HEIGHT) {
        for j in range(0, WIDTH) {
            let x: f32 = vec_x.get(i, j);
            let y: f32 = vec_y.get(i, j);
            let z: f32 = vec_z.get(i, j);

            writer.write(format!("{} {} {} ", to_int(x), to_int(y), to_int(z)).as_bytes()).ok();
        }
    }
}
