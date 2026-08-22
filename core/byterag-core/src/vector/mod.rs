pub mod flat_index;
pub mod simd;

pub use flat_index::{FlatVectorIndex, VectorSearchResult};
pub use simd::{Metric, cosine_similarity, dot_product, l2_distance};
