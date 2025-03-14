mod color;
pub use color::Color as RbColor;

mod tone;
pub use tone::Tone as RbTone;

mod table;
pub use table::Table as RbTable;

mod rect;
pub use rect::Rect as RbRect;

pub fn bind(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
    color::bind(ruby)?;
    tone::bind(ruby)?;
    table::bind(ruby)?;
    rect::bind(ruby)?;

    Ok(())
}
