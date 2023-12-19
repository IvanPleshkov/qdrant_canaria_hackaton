use crate::Error;

pub struct ScoredIndex {
    pub score: f32,
    pub point: usize,
}

pub enum QDrant {
    Local(LocalQDrant),
}

pub struct LocalQDrant {
    pub vectors: Vec<Vec<f32>>,
}

impl QDrant {
    pub fn new() -> Result<Self, Error> {
        Ok(QDrant::Local(LocalQDrant { vectors: vec![] }))
    }

    pub fn recreate(&mut self) {
        match self {
            QDrant::Local(qdrant) => {
                qdrant.vectors.clear();
            }
        }
    }

    pub fn insert(&mut self, vector: Vec<f32>) -> Result<(), Error> {
        match self {
            QDrant::Local(qdrant) => {
                qdrant.vectors.push(vector);
                Ok(())
            }
        }
    }

    pub fn search(&self, query: &[f32], top: usize) -> Result<Vec<ScoredIndex>, Error> {
        match self {
            QDrant::Local(qdrant) => {
                let mut scored_indices: Vec<ScoredIndex> = qdrant
                    .vectors
                    .iter()
                    .enumerate()
                    .map(|(i, vector)| ScoredIndex {
                        score: cosine_similarity(query, vector),
                        point: i,
                    })
                    .collect();
                scored_indices.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
                Ok(scored_indices.into_iter().take(top).collect())
            }
        }
    }
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let mut dot = 0.0;
    let mut a_norm = 0.0;
    let mut b_norm = 0.0;
    for i in 0..a.len() {
        dot += a[i] * b[i];
        a_norm += a[i] * a[i];
        b_norm += b[i] * b[i];
    }
    dot / (a_norm.sqrt() * b_norm.sqrt())
}
