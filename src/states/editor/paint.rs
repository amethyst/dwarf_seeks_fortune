use crate::components::*;
use crate::levels::{load_transform, EntityType, Map, Tile, TileType};
use crate::resources::{Assets, EditorData, SpriteType};
use amethyst::prelude::Builder;
use amethyst::{
    core::math::Vector3,
    core::transform::Transform,
    prelude::{World, WorldExt},
    renderer::sprite::SpriteRender,
};
use std::cmp::min;

pub fn paint_tiles(world: &mut World) {
    let mut created: Vec<(Pos, TileType)> = vec![];
    {
        let editor_data = world.read_resource::<EditorData>();
        let mut map = world.write_resource::<Map>();
        let lower_bounds = (&*editor_data).selector.lower_bounds();
        let dimens = (&*editor_data).selector.dimens();
        for x in lower_bounds.x..(lower_bounds.x + dimens.x) {
            for y in lower_bounds.y..(lower_bounds.y + dimens.y) {
                created.push((Pos::new(x, y), (&*editor_data).brush.tile.clone()));
                (&mut *map).put_tile(Pos::new(x, y), (&*editor_data).brush.tile.clone());
            }
        }
    }
    for (pos, tile) in created {
        let mut transform = Transform::default();
        transform.set_scale(Vector3::new(1. / 128., 1. / 128., 1.0));
        transform.set_translation_xyz(pos.x as f32 + 0.5, pos.y as f32 + 0.5, 0.0);
        let sprite_handle = world
            .read_resource::<Assets>()
            .get_still(&SpriteType::Blocks);
        world
            .create_entity()
            .with(SpriteRender {
                sprite_sheet: sprite_handle.clone(),
                sprite_number: 0,
            })
            .with(transform)
            .with(PaintedTileTag)
            .build();
    }
}
