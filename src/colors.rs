use rltk::RGBA;



pub fn mix_colors(color_one: RGBA, color_two: RGBA, factor: f32) -> RGBA {
    /*if factor <= 0.0 {
        return color_one;
    }
    if factor >= 1.0 {
        return color_two;
    }*/

    // Convert to RGB
    let rgb1 = color_one.to_rgb();
    let rgb2 = color_two.to_rgb();

    // Check if color_two is white or very close to white
    /*if rgb2.r > 0.9 && rgb2.g > 0.9 && rgb2.b > 0.9 {
        // Brighten color_one
        let r = (rgb1.r + factor).min(1.0);
        let g = (rgb1.g + factor).min(1.0);
        let b = (rgb1.b + factor).min(1.0);
        let a = color_one.a + (color_two.a - color_one.a) * factor;
        return RGBA::from_f32(r, g, b, a);
    } */


 
    
    // Linear interpolation in RGB space for non-white colors
    let r = rgb1.r + (rgb2.r - rgb1.r) * factor;
    let g = rgb1.g + (rgb2.g - rgb1.g) * factor;
    let b = rgb1.b + (rgb2.b - rgb1.b) * factor;
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