use rltk::RGBA;
use serde::{ser::SerializeStruct, Serialize, Serializer};



/*pub fn mix_colors(color_one: RGBA, color_two: RGBA, factor: f32) -> RGBA {
    let r = color_one.r + (color_two.r - color_one.r) * factor;
    let g = color_one.g + (color_two.g - color_one.g) * factor;
    let b = color_one.b + (color_two.b - color_one.b) * factor;
    let a = color_one.a + (color_two.a - color_one.a) * factor;
    RGBA::from_f32(r, g, b, a)
}*/

pub fn mix_colors(color_one: RGBA, color_two: RGBA, factor: f32) -> RGBA {
    let value_one = color_one.to_rgb().to_hsv().v;
    let value_two = color_two.to_rgb().to_hsv().v;

    /*if color_two.r > 0.9 && color_two.g > 0.9 && color_two.b > 0.9 {
        let alpha = color_two.a;
        let mut color = color_two.to_rgb().to_hsv();

        color.v = color_one.to_rgb().to_hsv().v + (value_two - value_one) * factor;
        
        let color = color.to_rgba(alpha);

        return color;
    }
    else if color_one.r > 0.9 && color_one.g > 0.9 && color_one.b > 0.9 {
        let alpha = color_two.a;
        let mut color = color_two.to_rgb().to_hsv();

        color.v = color_one.to_rgb().to_hsv().v + (value_two - value_one) * factor;
        
        let color = color.to_rgba(alpha);

        return color;
    }
    else {*/
        let r = color_one.r + (color_two.r - color_one.r) * factor;
        let g = color_one.g + (color_two.g - color_one.g) * factor;
        let b = color_one.b + (color_two.b - color_one.b) * factor;
        let a = color_one.a + (color_two.a - color_one.a) * factor;
        RGBA::from_f32(r, g, b, a)
    //}
}

pub fn mix_surface_light_colors(color_one: RGBA, color_two: RGBA, factor: f32) -> RGBA {
    let value_one = color_one.to_rgb().to_hsv().v;
    let value_two = color_two.to_rgb().to_hsv().v;

    if color_two.r > 0.9 && color_two.g > 0.9 && color_two.b > 0.9 {
        let alpha = color_two.a;
        let mut color = color_one.to_rgb().to_hsv();

        color.v = color_one.to_rgb().to_hsv().v + (value_two - value_one) * factor;
        
        let color = color.to_rgba(alpha);

        return color;
    }
    else if color_one.r > 0.9 && color_one.g > 0.9 && color_one.b > 0.9 {
        let alpha = color_two.a;
        let mut color = color_two.to_rgb().to_hsv();

        color.v = color_one.to_rgb().to_hsv().v + (value_two - value_one) * factor;
        
        let color = color.to_rgba(alpha);

        return color;
    }


    else {
        let r = color_one.r + (color_two.r - color_one.r) * factor;
        let g = color_one.g + (color_two.g - color_one.g) * factor;
        let b = color_one.b + (color_two.b - color_one.b) * factor;
        let a = color_one.a;
        RGBA::from_f32(r, g, b, a)
    }
}

pub fn dim_color(color: RGBA, amount: f32) -> RGBA {
    let amount = amount.clamp(0.0, 1.0);
    let alpha = color.a;

    let mut color = color.to_rgb().to_hsv();

    color.v = color.v * amount;

    color.to_rgba(alpha)
}

pub struct SerialisableRGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl From<RGBA> for SerialisableRGBA {
    fn from(rgba: RGBA) -> Self {
        SerialisableRGBA {
            r: (rgba.r * 255.0) as u8,
            g: (rgba.g * 255.0) as u8,
            b: (rgba.b * 255.0) as u8,
            a: (rgba.a * 255.0) as u8,
        }
    }
}

impl From<SerialisableRGBA> for RGBA {
    fn from(s_rgba: SerialisableRGBA) -> Self {
        RGBA::from_u8(s_rgba.r, s_rgba.g, s_rgba.b, s_rgba.a)
    }
}

impl Serialize for SerialisableRGBA {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        let mut state = serializer.serialize_struct("Vector3i", 4)?;
    
        state.serialize_field("r", &self.r)?;
        state.serialize_field("g", &self.g)?;
        state.serialize_field("b", &self.b)?;
        state.serialize_field("a", &self.a)?;
        state.end()
    }
}