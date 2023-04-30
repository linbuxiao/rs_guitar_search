use crate::model::GuitarScore;

pub trait SearchEngine {
    fn search(&self, song_name: &str) -> Vec<GuitarScore>;
}
