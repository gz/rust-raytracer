# Raytracers in Rust

This repository contains a few very simple raytracer implementations written in Rust. They are listed here in increasing order of complexity:

 1. `raytracer_2d.rs`: 
    The simplest possible (?) raytracer that produces a 2D image with orthographic projection.
    ![raytracer_2d.rs image](https://raw.githubusercontent.com/gz/rust-raytracer/master/raytracer_2d.jpg "raytracer_2d output")


 2. `raytracer_pinhole.rs`: 
    Similar to `raytracer_2d.rs` but now we use a pinhole camera model for capturing the image. Also added a light source and implemented simple phong shading.
    ![raytracer_pinhole.rs image](https://raw.githubusercontent.com/gz/rust-raytracer/master/raytracer_pinhole.jpg "raytracer_pinhole output")


 3. `raytracer_pinhole_path.rs`
    Implements path-based ray tracing by recursively following rays
    up until a maximum depth of 4. Rays are sampled randomly using a 
    monte-carlo based approach.
    ![raytracer_pinhole_path.rs image](https://raw.githubusercontent.com/gz/rust-raytracer/master/raytracer_pinhole_path.jpg "raytracer_pinhole_path output")


The code is inspired by the book [Realistic Ray Tracing (2nd Edition)][2] by Peter Shirley and R. Keith Morley and the [smallpt][1] project.
    
[1]: http://www.kevinbeason.com/smallpt/
[2]: http://www.amazon.com/Realistic-Ray-Tracing-Second-Edition/dp/1568814615