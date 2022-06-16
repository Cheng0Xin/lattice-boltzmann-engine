use std::fmt::Display;

pub struct VelocitySet {
    velocity: Vec<i8>,
    weight: f64,
}

trait VecOperation {}

impl VelocitySet {
    pub fn new(v_x: i8, v_y: i8, v_z: i8, w: f64) -> VelocitySet {
        VelocitySet {
            velocity: vec![v_x, v_y, v_z],
            weight: w,
        }
    }
}

impl Display for VelocitySet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tmp = self
            .velocity
            .iter()
            .map(|x| x.to_string())
            .reduce(|acc, x| format!("{}, {}", acc, x))
            .unwrap();
        f.write_fmt(format_args!("({}) - {}", tmp, self.weight))
    }
}

impl VecOperation for VelocitySet {}
