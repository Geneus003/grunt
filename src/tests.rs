use super::*;

#[test]
fn gen_axis_tests() {
    let ax = types::Axis::generate_axis(1.0, 3.0, Some(1.0));
    assert_eq!(ax.get_axis().clone(), vec![1.0, 2.0, 3.0]);

    let ax = types::Axis::generate_axis(0.5, 3.0, Some(1.0));
    assert_eq!(ax.get_axis().clone(), vec![0.5, 1.5, 2.5]);

    let ax = types::Axis::generate_axis(0.5, 2.0, Some(0.5));
    assert_eq!(ax.get_axis().clone(), vec![0.5, 1.0, 1.5, 2.0]);

    let ax = types::Axis::generate_axis(0.5, 2.1, Some(0.5));
    assert_eq!(ax.get_axis().clone(), vec![0.5, 1.0, 1.5, 2.0]);

    let ax = types::Axis::generate_axis(0.1, 0.3, Some(0.2));
    assert_eq!(ax.get_axis().clone(), vec![0.1, 0.3]);

    let ax = types::Axis::generate_axis(-0.2, 0.2, Some(0.2));
    assert_eq!(ax.get_axis().clone(), vec![-0.2, 0.0, 0.2]);

    let ax = types::Axis::generate_axis(-0.2, 0.2, Some(1.0));
    assert_eq!(ax.get_axis().clone(), vec![-0.2]);

    let ax = types::Axis::generate_axis(-0.2, 0.2, Some(-0.2));
    assert_eq!(ax.get_axis().clone(), vec![]);

    let ax = types::Axis::generate_axis(0.5, 2.0, None);
    assert_eq!(ax.get_axis().clone(), vec![0.5, 1.5]);

    let ax = types::Axis::generate_axis(1i16, 2i16, None);
    assert_eq!(ax.get_axis().clone(), vec![1.0, 2.0]);

    let ax = types::Axis::generate_axis(1i16, 5i16, Some(2i16));
    assert_eq!(ax.get_axis().clone(), vec![1.0, 3.0, 5.0]);

    let ax = types::Axis::generate_axis(1i16, 6i16, Some(2i16));
    assert_eq!(ax.get_axis().clone(), vec![1.0, 3.0, 5.0]);
}
