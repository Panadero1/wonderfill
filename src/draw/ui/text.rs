use speedy2d::font::Font;

pub fn get_font() -> Font {
    Font::new(include_bytes!("../../../assets/font/negative-quinpix.ttf")).unwrap()
}
