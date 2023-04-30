use crate::model::GuitarScore;
use crate::spider::base::SearchEngine;
use crate::spider::jitashe::JitasheSearchEngine;

pub struct SearchCore {
    engines: Vec<Box<dyn SearchEngine>>,
}

impl SearchCore {
    pub fn new() -> Self {
        Self {
            engines: vec![Box::new(JitasheSearchEngine::new())],
        }
    }

    pub fn search(&self, song_name: &str) -> Vec<GuitarScore> {
        let mut result = vec![];
        for engine in &self.engines {
            result.extend(engine.search(song_name));
        }
        result
    }
}

#[test]
fn test_jitashe() {
    let core = SearchCore::new();
    let result = core.search("我记得");
    println!("{:?}", result);
}
