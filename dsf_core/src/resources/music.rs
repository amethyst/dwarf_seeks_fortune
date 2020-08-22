use amethyst::audio::SourceHandle;
use rand::prelude::SliceRandom;
use rand::thread_rng;

use std::iter::Cycle;
use std::vec::IntoIter;

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

impl Music {
    pub fn new(mut tracks: Vec<SourceHandle>) -> Self {
        tracks.shuffle(&mut thread_rng());
        let music = tracks.into_iter().cycle();
        Music { music }
    }
}
