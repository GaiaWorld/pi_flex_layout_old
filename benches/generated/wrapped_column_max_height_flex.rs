fn print(count: &mut usize, id: usize, layout: &layout::tree::LayoutR) {
    *count += 1;
    debug_println!("result: {:?} {:?} {:?}", *count, id, layout);
}
pub fn compute() {
    let mut layout_tree = layout::tree::LayoutTree::default();
    layout_tree.insert(
        1,
        0,
        0,
        layout::idtree::InsertType::Back,
        layout::style::Style {
            position_type: layout::style::PositionType::Absolute,
            size: layout::geometry::Size {
                width: layout::style::Dimension::Points(1920.0),
                height: layout::style::Dimension::Points(1024.0),
            },
            ..Default::default()
        },
    );
    layout_tree.insert(
        2,
        1,
        0,
        layout::idtree::InsertType::Back,
        layout::style::Style {
            flex_direction: layout::style::FlexDirection::Column,
            flex_wrap: layout::style::FlexWrap::Wrap,
            align_items: layout::style::AlignItems::Center,
            align_content: layout::style::AlignContent::Center,
            justify_content: layout::style::JustifyContent::Center,
            size: layout::geometry::Size {
                width: layout::style::Dimension::Points(700f32),
                height: layout::style::Dimension::Points(500f32),
                ..Default::default()
            },
            ..Default::default()
        },
    );
    layout_tree.insert(
        3,
        2,
        0,
        layout::idtree::InsertType::Back,
        layout::style::Style {
            flex_grow: 1f32,
            flex_shrink: 1f32,
            flex_basis: layout::style::Dimension::Percent(0f32),
            size: layout::geometry::Size {
                width: layout::style::Dimension::Points(100f32),
                height: layout::style::Dimension::Points(500f32),
                ..Default::default()
            },
            max_size: layout::geometry::Size {
                height: layout::style::Dimension::Points(200f32),
                ..Default::default()
            },
            ..Default::default()
        },
    );
    layout_tree.insert(
        4,
        2,
        0,
        layout::idtree::InsertType::Back,
        layout::style::Style {
            flex_grow: 1f32,
            flex_shrink: 1f32,
            flex_basis: layout::style::Dimension::Percent(0f32),
            size: layout::geometry::Size {
                width: layout::style::Dimension::Points(200f32),
                height: layout::style::Dimension::Points(200f32),
                ..Default::default()
            },
            margin: layout::geometry::Rect {
                start: layout::style::Dimension::Points(20f32),
                end: layout::style::Dimension::Points(20f32),
                top: layout::style::Dimension::Points(20f32),
                bottom: layout::style::Dimension::Points(20f32),
                ..Default::default()
            },
            ..Default::default()
        },
    );
    layout_tree.insert(
        5,
        2,
        0,
        layout::idtree::InsertType::Back,
        layout::style::Style {
            size: layout::geometry::Size {
                width: layout::style::Dimension::Points(100f32),
                height: layout::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
    );
    layout_tree.compute(print, &mut 0);
}