use std::time::Instant;
use wide::f32x8;

// 1. Scalar Cosine Similarity (Baseline)
fn cosine_similarity_scalar(a: &[f32], b: &[f32]) -> f32 {
    let mut dot = 0.0f32;
    let mut norm_a = 0.0f32;
    let mut norm_b = 0.0f32;
    for i in 0..a.len() {
        dot += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a.sqrt() * norm_b.sqrt())
    }
}

// 2. SIMD (f32x8 AVX2) Cosine Similarity
fn cosine_similarity_simd(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    let len = a.len();
    let chunks = len / 8;

    let mut dot_simd = f32x8::splat(0.0);
    let mut norm_a_simd = f32x8::splat(0.0);
    let mut norm_b_simd = f32x8::splat(0.0);

    for i in 0..chunks {
        let offset = i * 8;
        let va = f32x8::from(&a[offset..offset + 8]);
        let vb = f32x8::from(&b[offset..offset + 8]);

        dot_simd += va * vb;
        norm_a_simd += va * va;
        norm_b_simd += vb * vb;
    }

    let mut dot = dot_simd.reduce_add();
    let mut norm_a = norm_a_simd.reduce_add();
    let mut norm_b = norm_b_simd.reduce_add();

    // Remainder
    for i in (chunks * 8)..len {
        dot += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }

    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a.sqrt() * norm_b.sqrt())
    }
}

fn main() {
    println!("============================================================");
    println!("   DBX Vector SIMD Micro-Benchmark (OpenAI 1536-dim)       ");
    println!("============================================================");

    let dim = 1536;
    let num_vectors = 10_000;
    println!("[1] Generating {} vectors (dim={})...", num_vectors, dim);

    // Dummy vectors
    let query_vector: Vec<f32> = (0..dim).map(|i| (i as f32 * 0.001).sin()).collect();
    let mut dataset: Vec<Vec<f32>> = Vec::with_capacity(num_vectors);
    for i in 0..num_vectors {
        let v: Vec<f32> = (0..dim).map(|j| ((i + j) as f32 * 0.001).cos()).collect();
        dataset.push(v);
    }

    // Benchmark Scalar Brute-Force Scan
    let start_scalar = Instant::now();
    let mut max_sim_scalar = -1.0f32;
    for v in &dataset {
        let sim = cosine_similarity_scalar(&query_vector, v);
        if sim > max_sim_scalar {
            max_sim_scalar = sim;
        }
    }
    let scalar_dur = start_scalar.elapsed();
    println!(
        "  >> Scalar Scan (10K x 1536-dim): {:.2?} (max_sim: {:.4})",
        scalar_dur, max_sim_scalar
    );

    // Benchmark SIMD (f32x8) Brute-Force Scan
    let start_simd = Instant::now();
    let mut max_sim_simd = -1.0f32;
    for v in &dataset {
        let sim = cosine_similarity_simd(&query_vector, v);
        if sim > max_sim_simd {
            max_sim_simd = sim;
        }
    }
    let simd_dur = start_simd.elapsed();
    println!(
        "  >> SIMD f32x8 Scan (10K x 1536-dim): {:.2?} (max_sim: {:.4})",
        simd_dur, max_sim_simd
    );

    let speedup = scalar_dur.as_secs_f64() / simd_dur.as_secs_f64();
    println!("============================================================");
    println!("   SIMD Acceleration Speedup: {:.2}x faster", speedup);
    println!(
        "   Per Vector Search Latency: {:.2} ns/vec",
        (simd_dur.as_nanos() as f64) / (num_vectors as f64)
    );
    println!("============================================================");
}
