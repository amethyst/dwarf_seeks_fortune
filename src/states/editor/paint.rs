use crate::components::*;
use crate::levels::*;
use crate::resources::{EditorData, TileEdit};

use amethyst::prelude::{World, WorldExt};

pub fn paint_tiles(world: &mut World) {
    let (key, tile_def) = get_brush(world);
    set_tiles(world, key, tile_def);
}

pub fn erase_tiles(world: &mut World) {
    set_tiles(world, None, None);
}

fn set_tiles(world: &mut World, key: Option<String>, tile_def: Option<TileDefinition>) {
    let brush_dimens = tile_def
        .as_ref()
        .map(|def| def.dimens)
        .unwrap_or(Pos::new(1, 1));
    let mut editor_data = world.write_resource::<EditorData>();
    let lower_bounds = (&*editor_data).selector.lower_bounds();
    let selection_dimens = (&*editor_data).selector.dimens();
    for x in
        (lower_bounds.x..(lower_bounds.x + selection_dimens.x)).step_by(brush_dimens.x as usize)
    {
        for y in
            (lower_bounds.y..(lower_bounds.y + selection_dimens.y)).step_by(brush_dimens.y as usize)
        {
            (&mut *editor_data)
                .level
                .put_tile(Pos::new(x, y), key.clone().map(|key| TileEdit::new(key)));
        }
    }
}

fn get_brush(world: &World) -> (Option<String>, Option<TileDefinition>) {
    let key = (&*world.write_resource::<EditorData>())
        .brush
        .get_key()
        .clone();
    let def = key
        .as_ref()
        .map(|key| world.write_resource::<TileDefinitions>().get(key).clone());
    (key, def)
}
