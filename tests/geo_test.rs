#[test]
fn distance_zero() {
    let d = attendance_server::utils::geo::calculate_distance(30.0, 120.0, 30.0, 120.0);
    assert!(d.abs() < 1e-6);
}

#[test]
fn distance_nonzero() {
    let d = attendance_server::utils::geo::calculate_distance(30.0, 120.0, 30.001, 120.001);
    assert!(d > 0.0);
}
