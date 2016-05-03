extern crate convolution;
extern crate threadpool;
use std::sync::Arc;
use convolution::convolute::*;
use convolution::utils::*;
use std::thread;


use std::env;

// Prints each argument on a separate line


fn main() {

    const X: usize = 4;
    const Y: usize = 4;

    //  Getting image path and stencils
    let args = env::args();

    let mut num_cpus = 1;
    let mut image_path = "".to_string();
    let mut stencils = Vec::new();

    for (i, j) in args.enumerate(){
        if i == 1 {
            let cpu_input = j.clone().parse::<usize>().unwrap();
            if cpu_input > 1 {
                num_cpus = cpu_input
            }
        }

        if i == 2  {
            image_path = j.clone()
        }

        if i >= 3 {
            stencils.push(j.clone())
        }
    }

    let loaded_img = img_from_file(image_path);


    let stencil: Vec<Vec<f32>> = stencil_from_file(stencils.first().unwrap());

    let shared_stencil =  Arc::new(stencil);
    let shared_img =  Arc::new(loaded_img);

    static mut output_image: [[(f32, f32, f32);X];Y] = [[(0.0,0.0,0.0); X]; Y];


    let mut handles = Vec::new();

    for i in 0..num_cpus {
        let stencil_ref = shared_stencil.clone();
        let img_ref = shared_img.clone();
        println!("{:?}", i);


        let handle = thread::spawn(move || {
            unsafe {
                let result = compute_pixel(&stencil_ref, Coordinate(i as isize,i as isize), &img_ref.clone());

                output_image[i as usize][i as usize]= (result.0, result.1, result.2);
                println!("{:?}", result);
            }
        });
        handles.push(handle);
    }

    for h in handles {
        h.join();
    }

    unsafe{
        println!("{:?}", output_image);
    }


    // println!("Pixel value computed: {}", result[0].0);
}
