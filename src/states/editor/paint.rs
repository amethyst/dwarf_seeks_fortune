use crate::components::*;
use crate::levels::*;
use crate::resources::{Assets, EditorData, SpriteType, TileEdit};
use amethyst::prelude::Builder;
use amethyst::{
    core::math::Vector3,
    core::transform::Transform,
    prelude::{World, WorldExt},
    renderer::sprite::SpriteRender,
};
use std::cmp::min;

pub fn paint_tiles(world: &mut World) {
    {
        let mut editor_data = world.write_resource::<EditorData>();
        let lower_bounds = (&*editor_data).selector.lower_bounds();
        let dimens = (&*editor_data).selector.dimens();
        for x in lower_bounds.x..(lower_bounds.x + dimens.x) {
            for y in lower_bounds.y..(lower_bounds.y + dimens.y) {
                let key = (&*editor_data).brush.get_key().clone();
                (&mut *editor_data)
                    .level
                    .put_tile(Pos::new(x, y), key.map(|key| TileEdit::new(key)));
            }
        }
    }
}
