use soloud::*;

use std::path::Path;

pub struct GameSFX {
    soloud: Soloud,
    collision_wav: Wav,
    score_wav: Wav,
}

impl GameSFX {
    pub fn new() -> GameSFX {
        let sl = Soloud::default().expect("Failed to get Soloud object!");

        let mut sounds = GameSFX {
            soloud: sl,
            collision_wav: audio::Wav::default(),
            score_wav: audio::Wav::default(),
        };

        let sfx_collision_path: &Path = std::path::Path::new("asset/sfx/pew.wav");
        let sfx_score_path: &Path = std::path::Path::new("asset/sfx/score.mp3");
        sounds
            .collision_wav
            .load(sfx_collision_path)
            .unwrap_or_else(|_| {
                panic!(
                    "failed to load sfx file {} for collision",
                    sfx_collision_path.display()
                )
            });
        sounds.score_wav.load(sfx_score_path).unwrap_or_else(|_| {
            panic!(
                "failed to load sfx file {} for score",
                sfx_score_path.display()
            )
        });
        sounds
    }

    pub fn play_collision(&self) {
        self.soloud.play(&self.collision_wav);
    }

    pub fn play_score(&self) {
        self.soloud.play(&self.score_wav);
    }
}
