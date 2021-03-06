// x, y
pub struct Coordinate(pub isize, pub isize);

// In the stencil and the image, the first coordinate is x, the second is y.
// This is computing the convolution between the stencil and the extracted matrix from the
// image
pub fn compute_pixel(stencil: &Vec<Vec<f32>>, coordinate: Coordinate, image_ref: &Vec<Vec<(f32, f32, f32)>>) -> (f32, f32, f32) {
    let stencil_size = (stencil.len(), stencil[0].len());

    let extracted_matrix = extract_matrix(image_ref, coordinate, stencil_size);

    let mut result: (f32, f32, f32) = (0.0, 0.0, 0.0);

    for i in 0..stencil_size.1 {
        for j in 0..stencil_size.0 {
            result.0 += extracted_matrix[i][j].0 * stencil[i][j];
            result.1 += extracted_matrix[i][j].1 * stencil[i][j];
            result.2 += extracted_matrix[i][j].2 * stencil[i][j];
        }
    }
    result
}

// Extracts the pixels values for a given stencil dimensions from an image
pub fn extract_matrix(image_ref: &Vec<Vec<(f32, f32, f32)>>, coordinate: Coordinate, dimensions: (usize, usize))
-> Vec<Vec<(f32, f32, f32)>>{

    let delta_x = ((dimensions.0 - 1) as f32 / 2.0) as isize;
    let delta_y = ((dimensions.1 - 1) as f32 / 2.0) as isize;

    let width = image_ref[0].len() as isize;
    let height = image_ref.len() as isize;

    let mut extracted_matrix: Vec<Vec<(f32, f32, f32)>> = Vec::with_capacity(dimensions.1);

    for (i, y) in ((coordinate.1 - delta_y)..(coordinate.1 + delta_y + 1)).enumerate() {
        extracted_matrix.push(Vec::with_capacity(dimensions.0));
        for x in (coordinate.0 - delta_x)..(coordinate.0 + delta_x + 1) {
            if x < 0 || y < 0 || x >= width || y >= height {
                extracted_matrix[i].push((0.0, 0.0, 0.0));
            } else {
                extracted_matrix[i].push(image_ref[y as usize][x as usize]);
            }
        }
    }

    extracted_matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_regular_extraction() {
        let image: Vec<Vec<(f32, f32, f32)>>= vec!(
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0))
        );

        let stencil: Vec<Vec<f32>> = vec!(
            vec!(1.0,3.0,4.0),
            vec!(1.0,3.0,4.0),
            vec!(1.0,3.0,4.0)
        );
        let stencil_size = (stencil.len(), stencil[0].len());
        let coord = Coordinate(1, 1);
        let extracted = extract_matrix(&image, coord, stencil_size);

        assert_eq!(extracted.len(), 3);
        assert_eq!(extracted[0].len(), 3);
        assert_eq!(extracted[0][0], (1.0,3.0,4.0));
        assert_eq!(extracted[0][1], (5.0,6.0,3.0));
        assert_eq!(extracted[1][0], (1.0,3.0,4.0));
        assert_eq!(extracted[1][1], (5.0,6.0,3.0));
        assert_eq!(extracted[2][2], (4.0,5.0,6.0));
    }

    #[test]
    fn matrix_lower_border_extraction() {
        let image: Vec<Vec<(f32, f32, f32)>>= vec!(
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0))
        );

        let stencil: Vec<Vec<f32>> = vec!(
            vec!(1.0,3.0,4.0),
            vec!(1.0,3.0,4.0),
            vec!(1.0,3.0,4.0)
        );
        let stencil_size = (stencil.len(), stencil[0].len());
        let coord = Coordinate(0, 0);
        let extracted = extract_matrix(&image, coord, stencil_size);

        assert_eq!(extracted.len(), 3);
        assert_eq!(extracted[0].len(), 3);
        assert_eq!(extracted[0][0], (0.0,0.0,0.0));
        assert_eq!(extracted[0][1], (0.0,0.0,0.0));
        assert_eq!(extracted[1][0], (0.0,0.0,0.0));
        assert_eq!(extracted[1][1], (1.0,3.0,4.0));
        assert_eq!(extracted[2][2], (5.0,6.0,3.0));
    }

    #[test]
    fn matrix_higher_border_extraction() {
        let image: Vec<Vec<(f32, f32, f32)>>= vec!(
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0))
        );

        let stencil: Vec<Vec<f32>> = vec!(
            vec!(1.0,3.0,4.0),
            vec!(1.0,3.0,4.0),
            vec!(1.0,3.0,4.0)
        );
        let stencil_size = (stencil.len(), stencil[0].len());
        let coord = Coordinate(2, 8);
        let extracted = extract_matrix(&image, coord, stencil_size);

        assert_eq!(extracted.len(), 3);
        assert_eq!(extracted[0].len(), 3);
        assert_eq!(extracted[0][0], (5.0, 6.0, 3.0));
        assert_eq!(extracted[0][1], (4.0, 5.0, 6.0));
        assert_eq!(extracted[1][0], (5.0, 6.0, 3.0));
        assert_eq!(extracted[1][1], (4.0, 5.0, 6.0));
        assert_eq!(extracted[2][2], (0.0, 0.0, 0.0));
    }

    #[test]
    fn compute_pixel_test() {
        let image: Vec<Vec<(f32, f32, f32)>>= vec!(
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0)),
            vec!((1.0,3.0,4.0), (5.0,6.0,3.0), (4.0,5.0,6.0))
        );

        let stencil: Vec<Vec<f32>> = vec!(
            vec!(1.0,3.0,4.0),
            vec!(1.0,3.0,4.0),
            vec!(1.0,3.0,4.0)
        );
        let coord = Coordinate(1, 1);
        let pixel_value = compute_pixel(&stencil, coord, &image);

        assert_eq!(pixel_value, (96.0, 123.0, 111.0));
    }

    #[test]
    fn compute_mono_pixel_test() {
        let image: Vec<Vec<(f32, f32, f32)>>= vec!(vec!((1.0,3.0,4.0)));

        let stencil: Vec<Vec<f32>> = vec!(
            vec!(1.0,3.0,4.0),
            vec!(1.0,3.0,4.0),
            vec!(1.0,3.0,4.0)
        );
        let coord = Coordinate(0, 0);
        let pixel_value = compute_pixel(&stencil, coord, &image);

        assert_eq!(pixel_value, (3.0, 9.0, 12.0));
    }
}
