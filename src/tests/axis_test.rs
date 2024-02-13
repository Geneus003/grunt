use super::types::Axis;

#[test]
fn gen_axis_tests() {
    let ax = Axis::generate_axis(1.0, 3.0, Some(1.0));
    assert_eq!(*ax.get_axis(), vec![1.0, 2.0, 3.0]);

    let ax = Axis::generate_axis(0.5, 3.0, Some(1.0));
    assert_eq!(*ax.get_axis(), vec![0.5, 1.5, 2.5]);

    let ax = Axis::generate_axis(0.5, 2.0, Some(0.5));
    assert_eq!(*ax.get_axis(), vec![0.5, 1.0, 1.5, 2.0]);

    let ax = Axis::generate_axis(0.5, 2.1, Some(0.5));
    assert_eq!(*ax.get_axis(), vec![0.5, 1.0, 1.5, 2.0]);

    let ax = Axis::generate_axis(0.1, 0.3, Some(0.2));
    assert_eq!(*ax.get_axis(), vec![0.1, 0.3]);

    let ax = Axis::generate_axis(-0.2, 0.2, Some(0.2));
    assert_eq!(*ax.get_axis(), vec![-0.2, 0.0, 0.2]);

    let ax = Axis::generate_axis(-0.2, 0.2, Some(1.0));
    assert_eq!(*ax.get_axis(), vec![-0.2]);

    let ax = Axis::generate_axis(-0.2, 0.2, Some(-0.2));
    assert_eq!(*ax.get_axis(), vec![]);

    let ax = Axis::generate_axis(0.2, -0.2, Some(-0.2));
    assert_eq!(*ax.get_axis(), vec![0.2, 0.0, -0.2]);

    let ax = Axis::generate_axis(0.2, -1.2, None);
    assert_eq!(*ax.get_axis(), vec![0.2, -0.8]);

    let ax = Axis::generate_axis(0.2, -0.3, None);
    assert_eq!(*ax.get_axis(), vec![0.2]);

    let ax = Axis::generate_axis(0.5, 2.0, None);
    assert_eq!(*ax.get_axis(), vec![0.5, 1.5]);

    let ax = Axis::generate_axis(1i16, 2i16, None);
    assert_eq!(*ax.get_axis(), vec![1.0, 2.0]);

    let ax = Axis::generate_axis(1i16, 5i16, Some(2i16));
    assert_eq!(*ax.get_axis(), vec![1.0, 3.0, 5.0]);

    let ax = Axis::generate_axis(1i16, 6i16, Some(2i16));
    assert_eq!(*ax.get_axis(), vec![1.0, 3.0, 5.0]);

    let ax = Axis::generate_axis(2i16, -2i16, None);
    assert_eq!(*ax.get_axis(), vec![2.0, 1.0, 0.0, -1.0, -2.0]);
}

#[test]
fn create_from_vec_tests() {
    let ax = Axis::create_from_vec(vec![1.0, 2.0, 3.1]).unwrap();
    assert_eq!(*ax.get_axis(), vec![1.0, 2.0, 3.1]);

    let ax = Axis::create_from_vec(vec![]);
    assert!(ax.is_err());
} 
