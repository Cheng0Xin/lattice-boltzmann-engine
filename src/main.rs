pub mod lbm_structure;

use lbm_structure as lbm;



fn main() {
    let x = lbm::VelocitySet::new(1, -1, 1, 4.0);
    println!("Hello, world!, {}", x);
}
