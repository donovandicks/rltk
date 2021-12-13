use rltk::{FontCharType, RGB};
use specs::{Component, DenseVecStorage};
use specs_derive::Component;

#[derive(Component)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}
