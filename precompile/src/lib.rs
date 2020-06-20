#![allow(
    dead_code,
    unused_must_use,
    unused_imports,
    unused_variables,
    unused_parens,
    unused_mut
)]

mod bundles;
mod startup;
mod structs;

pub use self::bundles::{PrecompiledDefaultsBundle, PrecompiledRenderBundle};
pub use self::structs::{AnimationId, MyPrefabData};
