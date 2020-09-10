use crate::resources::{DeprecatedEditorData, TileEdit};
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::ecs::{Read, System, Write};
use amethyst::input::{InputEvent, StringBindings, VirtualKeyCode};
use dsf_core::components::Pos;
use dsf_core::resources::{EventReaders, TileDefinition, TileDefinitions};

pub struct PlaceTilesSystem;

/// TODO: Delay in channel is unacceptable here. Replace channel with direct input check.
impl<'s> System<'s> for PlaceTilesSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Write<'s, EventReaders>,
        Read<'s, EventChannel<InputEvent<StringBindings>>>,
        Read<'s, TileDefinitions>,
        Write<'s, DeprecatedEditorData>,
    );

    fn run(&mut self, (mut readers, event_channel, tile_defs, mut editor_data): Self::SystemData) {
        let reader_id = readers
            .get_reader_id("place_tiles_system")
            .expect("ReaderId was not registered for system PlaceTilesSystem.");
        for event in event_channel.read(reader_id) {
            match event {
                InputEvent::KeyReleased {
                    key_code: VirtualKeyCode::Return,
                    scancode: _,
                } => {
                    let (key, tile_def) = get_brush(&editor_data, &tile_defs);
                    set_tiles(&mut editor_data, key, tile_def);
                }
                InputEvent::KeyReleased {
                    key_code: VirtualKeyCode::Delete,
                    scancode: _,
                } => {
                    set_tiles(&mut editor_data, None, None);
                }
                _ => (),
            }
        }
    }
}

fn set_tiles(
    editor_data: &mut DeprecatedEditorData,
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
            (*editor_data)
                .level
                .put_tile(Pos::new(x, y), key.clone().map(TileEdit::new));
        }
    }
    (*editor_data).level.dirty_everything()
}

fn get_brush(
    editor_data: &DeprecatedEditorData,
    tile_defs: &TileDefinitions,
) -> (Option<String>, Option<TileDefinition>) {
    let key = editor_data.brush.get_key().clone();
    let def = key.as_ref().map(|key| tile_defs.get(key).clone());
    (key, def)
}
