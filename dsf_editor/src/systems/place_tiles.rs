use crate::resources::{EditorData, LevelEdit};
use amethyst::core::ecs::{Read, System, Write};
use amethyst::input::{InputHandler, StringBindings};
use dsf_core::components::Pos;
use dsf_core::resources::{SignalEdge, SignalEdgeDetector, Tile, TileDefinition, TileDefinitions};

/// Responsible for placing and removing tiles based on player input.
pub struct PlaceTilesSystem;

impl<'s> System<'s> for PlaceTilesSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Write<'s, SignalEdgeDetector>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, EditorData>,
        Write<'s, LevelEdit>,
    );

    fn run(&mut self, (mut sed, input, editor_data, mut level_edit): Self::SystemData) {
        if let SignalEdge::Rising = sed.edge("place_blocks", &input) {
            let (key, tile_def) = get_brush(&editor_data, &level_edit.tile_map.tile_defs);
            set_tiles(&editor_data, &mut level_edit, key, tile_def);
        }
        if let SignalEdge::Rising = sed.edge("delete_blocks", &input) {
            set_tiles(&editor_data, &mut level_edit, None, None);
        }
    }
}

fn set_tiles(
    editor_data: &EditorData,
    level_edit: &mut LevelEdit,
    key: Option<String>,
    tile_def: Option<TileDefinition>,
) {
    let brush_dimens = tile_def
        .as_ref()
        .map(|def| def.dimens)
        .unwrap_or_else(|| Pos::new(1, 1));
    let lower_bounds = (*editor_data).selection.lower_bounds();
    let selection_dimens = (*editor_data).selection.dimens();
    for x in
        (lower_bounds.x..(lower_bounds.x + selection_dimens.x)).step_by(brush_dimens.x as usize)
    {
        for y in
            (lower_bounds.y..(lower_bounds.y + selection_dimens.y)).step_by(brush_dimens.y as usize)
        {
            (*level_edit).put_tile(false, Pos::new(x, y), key.clone().map(Tile::TileDefKey));
        }
    }
}

fn get_brush(
    editor_data: &EditorData,
    tile_defs: &TileDefinitions,
) -> (Option<String>, Option<TileDefinition>) {
    let key = editor_data.brush.get_key().clone();
    let def = key.as_ref().map(|key| tile_defs.get(key).clone());
    (key, def)
}
