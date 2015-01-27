#![feature(box_syntax)]

extern crate opencl;

use std::io::{BufferedWriter, File};
use opencl::mem::CLBuffer;
use std::fmt;
use std::num::Float;


#[derive(Show, Copy, Clone, Default)]
struct Vector {
    x: f32,
    y: f32,
    z: f32
}

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


const WIDTH: usize = 1024;
const HEIGHT: usize = 768;

fn main()
{
    let ker = include_str!("raytracer.ocl");
    println!("ker {}", ker);

    let (device, ctx, queue) = opencl::util::create_compute_context().unwrap();
    println!("{}", device.name());

    let c: CLBuffer<Vector> = ctx.create_buffer(1024*768, opencl::cl::CL_MEM_WRITE_ONLY);

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

    kernel.set_arg(0, &c);

    let event = queue.enqueue_async_kernel(&kernel, (1024is, 768is), None, ());

    let vec_c: Vec<Vector> = queue.get(&c, &event);

    println!("\nWriting Image...");
    let file = File::create(&Path::new("image.ppm"));
    let mut writer = BufferedWriter::new(file);

    writer.write(format!("P3\n{} {}\n{}\n", WIDTH, HEIGHT, 255).as_bytes()).ok();
    for i in range(0, HEIGHT) {
        for j in range(0, WIDTH) {
            let color: Vector = vec_c[i*WIDTH+j];
            writer.write(format!("{} {} {} ", to_int(color.x), to_int(color.y), to_int(color.z)).as_bytes()).ok();
        }
    }
}

fn string_from_slice<T: fmt::String>(slice: &[T]) -> String {
    let mut st = String::from_str("[");
    let mut first = true;

    for i in slice.iter() {
        if !first {
            st.push_str(", ");
        }
        else {
            first = false;
        }
        st.push_str(&*i.to_string())
    }

    st.push_str("]");
    return st
}