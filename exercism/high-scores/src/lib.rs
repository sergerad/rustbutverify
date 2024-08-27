#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().fold(None, |acc, &score| match acc {
            None => Some(score),
            Some(best) => Some(best.max(score)),
        })
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_three = self.scores.clone();
        top_three.sort();
        top_three.truncate(top_three.len().min(3));
        top_three
    }
}
