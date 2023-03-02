
use ab_glyph::{FontRef, Font, Glyph, point};


fn main() {

    // Get font static byte array compiled in
    let font_bytes = include_bytes!("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf");
    println!("font_bytes:{}", font_bytes.len());

    // Parses font
    let fref = match FontRef::try_from_slice(font_bytes) {
        Ok(f) => f,
        Err(err) => {
            panic!("Error parsing font:{}", err);
        },
    };

    // Show info about font
    println!("glyph_count :{}", fref.glyph_count());
    if let Some(v) = fref.units_per_em() {
        println!("units_per_em:{}", v);
    }
    println!("ascent_unscaled :{}", fref.ascent_unscaled());
    println!("descent_unscaled:{}", fref.descent_unscaled());
    println!("height_unscaled: {}", fref.height_unscaled());

    // Draw glyph
    let q_glyph: Glyph = fref.glyph_id('q').with_scale(12.0);
    if let Some(q) = fref.outline_glyph(q_glyph) {
        q.draw(|x, y, c| {
            println!("x:{x} y:{y} c:{c}");
        });
    }
}
