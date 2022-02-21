// matrix_util.rs

#[cfg(test)]
mod tests {
    use ultraviolet::{Mat4, Similarity3, Vec3};
    // use super::*;

    fn print_matrix(mat: &Mat4) {
        for col in mat.transposed().cols {
            let [a, b, c, d] = col.as_array();
            println!("[ {:12} , {:12} , {:12} , {:12} ]", a, b, c, d);
        }
    }

    #[test]
    fn known_transform() {
        let inp = Vec3::new(0.75, 0.75, 0.0);
        let mut similarity = Similarity3::identity();
        dbg!(inp);

        // similarity.prepend_translation(Vec3::new(-0.5, -0.5, 0.0));
        // dbg!(similarity.transform_vec(inp));

        similarity.append_translation(Vec3::new(-0.5, -0.5, 0.0));
        similarity.append_scaling(2.0);
        similarity.append_translation(Vec3::new(0.5, 0.5, 0.0));
        dbg!(similarity.transform_vec(inp));
        
        similarity.append_translation(Vec3::new(-0.25, -0.25, 0.0));
        dbg!(similarity.transform_vec(inp));

        similarity.append_translation(Vec3::new(-0.5, -0.5, 0.0));
        // similarity.append_scaling(1.0 / 0.75);
        similarity.append_scaling(2.0);
        similarity.append_translation(Vec3::new(0.5, 0.5, 0.0));
        dbg!(similarity.transform_vec(inp));
    }

    #[test]
    fn internal() {
        let dx = 0.5;
        let dy = 0.3;
        let zoom_factor = 2.0;

        dbg!(dx, dy);
        println!("translation only");
        let translation_mat = Mat4::from_translation(Vec3::new(dx, dy, 0.0));
        print_matrix(&translation_mat);
        println!();

        dbg!(zoom_factor);
        println!("zoom only");
        let zoom_mat = Mat4::from_scale(zoom_factor);
        print_matrix(&zoom_mat);
        println!();

        println!("zoom about middle");
        let zoom_middle_mat = Mat4::from_translation(Vec3::new(0.5, 0.5, 0.0))
            * zoom_mat
            * Mat4::from_translation(Vec3::new(-0.5, -0.5, 0.0));
        print_matrix(&zoom_middle_mat);
        println!();

        dbg!(dx, dy, zoom_factor);
        println!("all together");
        let transform_matrix = zoom_middle_mat * translation_mat;
        // let transform_matrix = translation_mat * zoom_middle_mat;
        print_matrix(&transform_matrix);
        println!();
        // print_matrix(
        //     &(Mat4::from_translation(Vec3::new(dx, dy, 0.0))
        //         * Mat4::from_translation(Vec3::new(0.5, 0.5, 0.0))
        //         * Mat4::from_scale(zoom_factor)
        //         * Mat4::from_translation(Vec3::new(-0.5, -0.5, 0.0))),
        // );
        // println!();
        println!();
        println!();

        let mut similarity = Similarity3::identity();

        // similarity.prepend_translation(Vec3::new(dx, dy, 0.0));
        // similarity.append_translation(Vec3::new(dx, dy, 0.0));

        similarity.append_translation(Vec3::new(-0.5, -0.5, 0.0));
        similarity.append_scaling(2.0 * zoom_factor);
        similarity.append_translation(Vec3::new(0.5, 0.5, 0.0));

        // similarity.append_translation(Vec3::new(-0.5, -0.5, 0.0));
        // similarity.append_scaling(zoom_factor);
        // similarity.append_translation(Vec3::new(0.5, 0.5, 0.0));

        // println!("zoom similarity");
        // print_matrix(&similarity.into_homogeneous_matrix());

        // similarity.prepend_translation(Vec3::new(dx, dy, 0.0));
        // similarity.append_translation(Vec3::new(dx, dy, 0.0));

        println!("full similarity");
        print_matrix(&similarity.into_homogeneous_matrix());

        // let points = vec![
        //     Vec3::new(0.0, 0.0, 1.0),
        //     Vec3::new(1.0, 0.0, 1.0),
        //     Vec3::new(1.0, 1.0, 1.0),
        //     Vec3::new(1.0, 0.0, 1.0),
        //     Vec3::new(0.5, 0.5, 1.0),
        // ];
        // for point in points {
        //     println!(
        //         "transform point  {:?}: {:?}",
        //         point,
        //         transform_matrix.transform_point3(point)
        //     );
        // }

        // println!(
        //     "transform vector (1, 1, 1): {:?}",
        //     transform_matrix.transform_vec3(Vec3::new(1.0, 1.0, 1.0))
        // );

        println!();
        println!();
    }
}
