pub mod simd;
pub mod flat_index;

pub use simd::{cosine_similarity, dot_product, l2_distance, Metric};
pub use flat_index::{FlatVectorIndex, VectorSearchResult};
