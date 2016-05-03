use std::io::prelude::*;
use std::fs::File;

pub fn img_from_file(image_path: String) -> Vec<Vec<(f32, f32, f32)>> {
    let mut f = File::open(image_path).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let rows: Vec<Vec<&str>> = s.split("\n").map(|x| x.split_whitespace().collect()).collect();

    let mut loaded_img: Vec<Vec<(f32, f32, f32)>> = Vec::new();

    for (i,j) in rows.iter().enumerate() {
        

        if i > 2 {
            let mut img_row: Vec<(f32, f32, f32)> = Vec::new();
            let chunks = j.chunks(3);

            for c in chunks {
                let mut rgb = (0.0, 0.0, 0.0);
                let values = c.iter().map(|x| x.parse::<f32>());

                for (ind,v) in values.enumerate() {
                    if ind == 0 { rgb.0 = v.clone().unwrap()}
                    if ind == 1 { rgb.1 = v.clone().unwrap()}
                    if ind == 2 { rgb.2 = v.clone().unwrap()}
                }
                img_row.push(rgb);
            }
            loaded_img.push(img_row);
        }
        
    }

    return loaded_img
}


pub fn stencil_from_file(stencil_path: &String) -> Vec<Vec<f32>> {
    let mut f = File::open(stencil_path).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let rows: Vec<Vec<&str>> = s.split("\n").map(|x| x.split_whitespace().collect()).collect();

    let mut loaded_stencil: Vec<Vec<f32>> = Vec::new();
    let mut m: f32 = 0.0;


    

    for (i,j) in rows.iter().enumerate() {
        
        if i == 2 {
            println!("{:?}", j);
            m = j[0].parse::<f32>().unwrap();
        }

        if i > 2 {
            loaded_stencil.push(j.iter().map(|x| -4.0+(8.0*x.parse::<f32>().unwrap())/(m-1.0)).collect());
        }
        
    }

    return loaded_stencil
}