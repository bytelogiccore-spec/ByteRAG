use std::cmp::Ordering;
use std::collections::BinaryHeap;
use super::simd::{cosine_similarity, dot_product, l2_distance, Metric};

#[derive(Debug, Clone)]
pub struct VectorSearchResult {
    pub id: Vec<u8>,
    pub score: f32,
    pub index: usize,
}

impl PartialEq for VectorSearchResult {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for VectorSearchResult {}

// Min-heap entry for Top-K tracking
#[derive(Clone)]
struct HeapEntry {
    score: f32,
    index: usize,
}

impl PartialEq for HeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for HeapEntry {}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Reverse for min-heap
        other.score.partial_cmp(&self.score)
    }
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

/// In-memory Flat SIMD Vector Index
pub struct FlatVectorIndex {
    pub dimension: usize,
    pub metric: Metric,
    pub ids: Vec<Vec<u8>>,
    pub vectors: Vec<f32>, // Contiguous buffer: len = N * dimension
}

impl FlatVectorIndex {
    pub fn new(dimension: usize, metric: Metric) -> Self {
        Self {
            dimension,
            metric,
            ids: Vec::new(),
            vectors: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    pub fn insert(&mut self, id: Vec<u8>, vector: &[f32]) {
        assert_eq!(vector.len(), self.dimension, "Dimension mismatch");
        self.ids.push(id);
        self.vectors.extend_from_slice(vector);
    }

    pub fn search(&self, query: &[f32], top_k: usize) -> Vec<VectorSearchResult> {
        assert_eq!(query.len(), self.dimension, "Query dimension mismatch");
        if self.is_empty() || top_k == 0 {
            return Vec::new();
        }

        let num_vectors = self.len();
        let mut heap: BinaryHeap<HeapEntry> = BinaryHeap::with_capacity(top_k);

        for i in 0..num_vectors {
            let offset = i * self.dimension;
            let vec_slice = &self.vectors[offset..offset + self.dimension];

            let score = match self.metric {
                Metric::Cosine => cosine_similarity(query, vec_slice),
                Metric::DotProduct => dot_product(query, vec_slice),
                Metric::L2 => -l2_distance(query, vec_slice), // Negative so closer distance = higher score
            };

            if heap.len() < top_k {
                heap.push(HeapEntry { score, index: i });
            } else if let Some(min_top) = heap.peek() {
                if score > min_top.score {
                    heap.pop();
                    heap.push(HeapEntry { score, index: i });
                }
            }
        }

        let mut results = Vec::with_capacity(heap.len());
        while let Some(entry) = heap.pop() {
            let final_score = if self.metric == Metric::L2 {
                -entry.score
            } else {
                entry.score
            };
            results.push(VectorSearchResult {
                id: self.ids[entry.index].clone(),
                score: final_score,
                index: entry.index,
            });
        }

        results.reverse();
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_vector_search() {
        let mut index = FlatVectorIndex::new(4, Metric::Cosine);
        index.insert(b"doc1".to_vec(), &[1.0, 0.0, 0.0, 0.0]);
        index.insert(b"doc2".to_vec(), &[0.0, 1.0, 0.0, 0.0]);
        index.insert(b"doc3".to_vec(), &[0.7071, 0.7071, 0.0, 0.0]);

        let query = vec![1.0, 0.0, 0.0, 0.0];
        let results = index.search(&query, 2);

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].id, b"doc1");
        assert!((results[0].score - 1.0).abs() < 1e-4);
        assert_eq!(results[1].id, b"doc3");
    }
}
