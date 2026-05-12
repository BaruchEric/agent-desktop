use agent_desktop_core::node::Rect;

use super::chain_steps::{center_is_inside, rect_has_area};

#[test]
fn rect_area_requires_positive_dimensions() {
    assert!(rect_has_area(&Rect {
        x: 0.0,
        y: 0.0,
        width: 1.0,
        height: 1.0,
    }));
    assert!(!rect_has_area(&Rect {
        x: 0.0,
        y: 0.0,
        width: 0.0,
        height: 1.0,
    }));
}

#[test]
fn center_visibility_uses_parent_bounds() {
    let outer = Rect {
        x: 10.0,
        y: 10.0,
        width: 100.0,
        height: 100.0,
    };
    assert!(center_is_inside(
        &Rect {
            x: 20.0,
            y: 20.0,
            width: 10.0,
            height: 10.0,
        },
        &outer
    ));
    assert!(!center_is_inside(
        &Rect {
            x: 200.0,
            y: 20.0,
            width: 10.0,
            height: 10.0,
        },
        &outer
    ));
}
