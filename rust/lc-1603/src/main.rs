pub mod solution;

use solution::ParkingSystem;

fn main() {
    let mut ps = ParkingSystem::new(1, 1, 0);
    assert_eq!(true, ps.add_car(1));
    assert_eq!(true, ps.add_car(2));
    assert_eq!(false, ps.add_car(3));
    assert_eq!(false, ps.add_car(1));
    assert_eq!(false, ps.add_car(2));
}
