fn main() {
    println!("Hello, world!");
}

// -------------
// Blow is Reserved for the matrix multiply

// x, y
struct Coordinate(usize, usize);

// In the stencil and the image, the first coordinate is x, the second is y.
fn compute_pixel(stencil: Vec<Vec<f32>>, coordinate: Coordinate, image_ref: &Vec<Vec<f32>>) {
    let stencil_size = (stencil.len(), stencil[0].len());

    let extracted_matrix = extract_matrix(image_ref, coordinate, stencil_size);


    for i in 0..stencil_size.0 {
        for j in 0..stencil_size.1 {
            for k in 0..stencil_size.1 {

            }
        }
    }
}

// Extracts the pixels values for a given stencil dimensions from an image
fn extract_matrix(image_ref: &Vec<Vec<f32>>, coordinate: Coordinate, dimensions: (usize, usize)){

    let delta_x = (dimensions.0 - 1)/2 as usize;
    let delta_y = (dimensions.1 - 1)/2 as usize;

    let mut extracted_matrix = Vec::with_capacity(dimensions.0);

    for (i, x) in ((coordinate.0 - delta_x)..(coordinate.0 + delta_x)).enumerate() {
        extracted_matrix.push(Vec::with_capacity(dimensions.1));
        for y in (coordinate.1 - delta_y)..(coordinate.1 + delta_y) {
            if x < 0 || y < 0 || x > image_ref.len() || y > image_ref[0].len() {
                extracted_matrix[i].push(0);
            } else {
                extracted_matrix[i].push(image_ref[x][y]);
            }
        }
    }
    extracted_matrix;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }
}
