use speedy2d::shape::Rectangle;

pub fn rect_from_size(width: u32, height: u32, pos: (u32, u32)) -> Rectangle {
    let half_width = width as f32 / 2.0;
    let half_height = height as f32 / 2.0;
    let x_pos = pos.0 as f32;
    let y_pos = pos.1 as f32;
    Rectangle::from_tuples((x_pos - half_width, y_pos - half_height), (x_pos + half_width, y_pos + half_height))
}