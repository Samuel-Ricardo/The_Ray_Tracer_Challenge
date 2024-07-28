use crate::{canvas::model::Canvas, tuple::model::Tuple};

pub enum Pixel {
    Coordinate { x: usize, y: usize },
    OutOfBounds,
}

impl Pixel {
    pub fn from_point_for_canvas(point: Tuple, canvas: &Canvas) -> Pixel {
        if !point.is_point() {
            panic!("Given tuple is not a point. Point needed for conversion to screen space.");
        }

        let rx = point.x.round();
        let ry = point.y.round();

        let ux = rx as usize;
        let uy = ry as usize;

        if rx.is_sign_negative() || ry.is_sign_negative() || ux > canvas.width || uy > canvas.height
        {
            return Pixel::OutOfBounds;
        }

        // INFO:Invert y axis to fit Screen space as the (0,0) coordinate is top left
        // and not bottom left

        let screen_x = ux;
        let screen_y = canvas.height - uy;

        Pixel::Coordinate {
            x: screen_x,
            y: screen_y,
        }
    }
}
