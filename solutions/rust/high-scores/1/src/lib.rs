#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {scores: scores.to_vec()}
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores[..]
    }

    pub fn latest(&self) -> Option<u32> {
        match self.scores.len() {
            0 => None,
            _ => Some(self.scores[self.scores.len() - 1])
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut cloned_scores = self.scores.to_vec();
        cloned_scores.sort();
        match cloned_scores.len() {
            0 => None,
            _ => Some(cloned_scores[cloned_scores.len() - 1])
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut cloned_scores = self.scores.to_vec();
        cloned_scores.sort();
        cloned_scores.iter().rev().take(3).map(|x| *x).collect()
    }
}
