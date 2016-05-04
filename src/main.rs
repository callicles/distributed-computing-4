extern crate convolution;
extern crate threadpool;
use std::sync::mpsc::channel;
use std::sync::Arc;
use convolution::convolute::*;
use convolution::utils::*;
use std::thread;


use std::env;

// Prints each argument on a separate line


fn main() {

    const x: usize = 4;
    const y: usize = 4;

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

    static mut output_image: [[(f32, f32, f32);x];y] = [[(0.0,0.0,0.0); x]; y];


    let mut handles = Vec::new();

    let x_range: Vec<usize> = (0..x).collect();
    let y_range: Vec<usize> = (0..y).collect();

    let mut xs = Vec::new();
    let mut ys = Vec::new();

    if num_cpus % 2 == 0 {
        xs = x_range.chunks(x/(num_cpus/2)).collect();
        ys = y_range.chunks(x/(num_cpus/2)).collect();
    } else {
        xs = x_range.chunks(x/(num_cpus/2 + 1)).collect();
        ys = y_range.chunks(x/(num_cpus/2 + 1)).collect();
    }

    println!("{:?}", xs.clone());
    println!("{:?}", ys.clone());

    let mut is = Vec::new();
    let mut js = Vec::new();

    for i in xs.clone() {
        let mut tmp = Vec::new();
        for n in i {
            tmp.push(n.clone())
        } 
        is.push(tmp.clone())
    }

    for j in ys.clone() {
        let mut tmp = Vec::new();
        for n in j {
            tmp.push(n.clone())
        }
        js.push(tmp.clone())
    }

    for i in is.clone() {
        for j in js.clone() {
            let arc_i = Arc::new(i.clone());
            let i_ref = arc_i.clone();
            
            let arc_j = Arc::new(j.clone());
            let j_ref = arc_j.clone();

            let stencil_ref = shared_stencil.clone();
            let img_ref = shared_img.clone();

            let handle = thread::spawn(move || {

                for a in &i_ref.clone()[..] {
                    for b in &j_ref.clone()[..] {
                        
                         unsafe {
                            let result = compute_pixel(&stencil_ref, Coordinate(*a as isize,*b as isize), &img_ref.clone());
                            
                            output_image[*a as usize][*b as usize]= (result.0, result.1, result.2);
                            
                        }
                    }
                }
            });

            handles.push(handle);
        }        
    }
    

    for h in handles {
        h.join();
    }
    
    unsafe{
        println!("{:?}", output_image);    
    }
    
}
