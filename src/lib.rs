pub mod de;
pub mod ser;

mod error;
mod tests;

#[derive(Clone, Copy)]
pub struct FormatOptions {
    pub short_length: bool,
    pub short_variant_index: bool,
}
