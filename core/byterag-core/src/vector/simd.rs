use wide::f32x8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Metric {
    Cosine,
    L2,
    DotProduct,
}

#[inline]
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    debug_assert_eq!(a.len(), b.len(), "Vector lengths must match");
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

    for i in (chunks * 8)..len {
        dot += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }

    if norm_a <= 0.0 || norm_b <= 0.0 {
        0.0
    } else {
        dot / (norm_a.sqrt() * norm_b.sqrt())
    }
}

#[inline]
pub fn l2_distance(a: &[f32], b: &[f32]) -> f32 {
    debug_assert_eq!(a.len(), b.len(), "Vector lengths must match");
    let len = a.len();
    let chunks = len / 8;

    let mut sum_sq_simd = f32x8::splat(0.0);

    for i in 0..chunks {
        let offset = i * 8;
        let va = f32x8::from(&a[offset..offset + 8]);
        let vb = f32x8::from(&b[offset..offset + 8]);
        let diff = va - vb;
        sum_sq_simd += diff * diff;
    }

    let mut sum_sq = sum_sq_simd.reduce_add();

    for i in (chunks * 8)..len {
        let diff = a[i] - b[i];
        sum_sq += diff * diff;
    }

    sum_sq.sqrt()
}

#[inline]
pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    debug_assert_eq!(a.len(), b.len(), "Vector lengths must match");
    let len = a.len();
    let chunks = len / 8;

    let mut dot_simd = f32x8::splat(0.0);

    for i in 0..chunks {
        let offset = i * 8;
        let va = f32x8::from(&a[offset..offset + 8]);
        let vb = f32x8::from(&b[offset..offset + 8]);
        dot_simd += va * vb;
    }

    let mut dot = dot_simd.reduce_add();

    for i in (chunks * 8)..len {
        dot += a[i] * b[i];
    }

    dot
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let v1 = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let v2 = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let sim = cosine_similarity(&v1, &v2);
        assert!((sim - 1.0).abs() < 1e-5);

        let v3 = vec![-1.0, -2.0, -3.0, -4.0, -5.0, -6.0, -7.0, -8.0, -9.0, -10.0];
        let sim_opp = cosine_similarity(&v1, &v3);
        assert!((sim_opp - (-1.0)).abs() < 1e-5);
    }

    #[test]
    fn test_l2_distance() {
        let v1 = vec![0.0; 16];
        let mut v2 = vec![0.0; 16];
        v2[0] = 3.0;
        v2[1] = 4.0;
        let dist = l2_distance(&v1, &v2);
        assert!((dist - 5.0).abs() < 1e-5);
    }
}
