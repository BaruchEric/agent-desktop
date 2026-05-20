mod common;

use common::{AdPoint, AdRect};

#[test]
fn rect_and_point_layouts_are_memcpyable() {
    let rect = AdRect {
        x: 1.25,
        y: -2.5,
        width: 640.0,
        height: 480.0,
    };
    let copied = unsafe { std::ptr::read(&rect as *const AdRect) };
    assert_eq!(copied.x, 1.25);
    assert_eq!(copied.y, -2.5);
    assert_eq!(copied.width, 640.0);
    assert_eq!(copied.height, 480.0);

    let point = AdPoint { x: 3.0, y: 4.0 };
    let copied = unsafe { std::ptr::read(&point as *const AdPoint) };
    assert_eq!(copied.x, 3.0);
    assert_eq!(copied.y, 4.0);
}
