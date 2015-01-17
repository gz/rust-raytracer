## Raytracers in Rust

[![Build Status](https://travis-ci.org/gz/rust-raytracer.svg)](https://travis-ci.org/gz/rust-raytracer)

This repository contains a few very simple raytracer implementations written in Rust. They are listed here in increasing order of complexity:

 1. `raytracer_2d.rs`: 
    The simplest possible (?) raytracer that produces a 2D image with orthographic projection.

    <img src="https://raw.githubusercontent.com/gz/rust-raytracer/master/raytracer_2d.jpg" height="100" width="100" >

 2. `raytracer_pinhole.rs`: 
    Similar to `raytracer_2d.rs` but now we use a pinhole camera model for capturing the image. Also added a light source and implemented simple phong shading.

    <img src="https://raw.githubusercontent.com/gz/rust-raytracer/master/raytracer_pinhole.jpg" height="100" width="100" >


 3. `raytracer_pinhole_path.rs`
    Implements path-based ray tracing by recursively following rays
    up until a maximum depth of 4. Rays are sampled randomly using a 
    monte-carlo based approach.

    |<img src="https://raw.githubusercontent.com/gz/rust-raytracer/master/raytracer_pinhole_path_5k.jpg" height="76" width="103" align="left">|<img src="https://raw.githubusercontent.com/gz/rust-raytracer/master/raytracer_pinhole_path_10k.jpg" height="76" width="103" >|
    |:---:|:---:|
    |5k samples|10k samples|

The code is inspired by the book [Realistic Ray Tracing (2nd Edition)][2] by Peter Shirley and R. Keith Morley and the [smallpt][1] project.
    
[1]: http://www.kevinbeason.com/smallpt/
[2]: http://www.amazon.com/Realistic-Ray-Tracing-Second-Edition/dp/1568814615
