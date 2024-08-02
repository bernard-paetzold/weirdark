use rltk::RGBA;



pub fn mix_colors(color_one: RGBA, color_two: RGBA, factor: f32) -> RGBA {

    let r = color_one.r + (color_two.r - color_one.r) * factor;
    let g = color_one.g + (color_two.g - color_one.g) * factor;
    let b = color_one.b + (color_two.b - color_one.b) * factor;
    let a = color_one.a + (color_two.a - color_one.a) * factor;
    RGBA::from_f32(r, g, b, a)
    
}

pub fn dim_color(color: RGBA, amount: f32) -> RGBA {
    let amount = amount.clamp(0.0, 1.0);
    let alpha = color.a;

    let mut color = color.to_rgb().to_hsv();

    color.v = color.v * amount;

    color.to_rgba(alpha)
}