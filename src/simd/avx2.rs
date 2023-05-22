///
///   AVX2 impls
#[cfg(all(
    feature = "use-avx2",
    target_feature = "avx2",
    any(target_arch = "x86_64", target_arch = "x86")
))]
pub fn is_sorted_avx2<T: num::Integer + crate::simd::SinglePrecision>(
    a: &[T],
    t: crate::simd::Trend,
) -> bool {
    use crate::simd::{is_sorted_scalar, Trend};
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    let a = a.as_ref();
    let n = a.len();
    let mut i = 0usize;
    if n >= 8 {
        unsafe {
            let compare = match t {
                Trend::Ascending => _mm256_cmpgt_epi32,
                Trend::Descending => move |a: __m256i, b: __m256i| _mm256_cmpgt_epi32(b, a),
            };
            let shuffle = _mm256_setr_epi32(1, 2, 3, 4, 5, 6, 7, 7);
            while i < n - 8 {
                let curr = _mm256_loadu_si256((&a[i..]).as_ptr() as *const _);
                let next = _mm256_permutevar8x32_epi32(curr, shuffle);

                let mask = compare(curr, next);

                if _mm256_testz_si256(mask, mask) != 1 {
                    return false;
                }
                i += 7;
            }
        }
    }

    is_sorted_scalar(a, n, i, t)
}

///
///   AVX2 impls
#[cfg(all(
    feature = "use-avx2",
    target_feature = "avx2",
    any(target_arch = "x86_64", target_arch = "x86")
))]
pub fn is_sorted_avx2_unroll4<T: num::Integer + crate::simd::SinglePrecision>(
    a: &[T],
    t: crate::simd::Trend,
) -> bool {
    use crate::simd::{is_sorted_scalar, Trend};
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    let a = a.as_ref();
    let n = a.len();
    let mut i = 0usize;

    if n >= 4 * 7 {
        unsafe {
            let compare = match t {
                Trend::Ascending => _mm256_cmpgt_epi32,
                Trend::Descending => move |a: __m256i, b: __m256i| _mm256_cmpgt_epi32(b, a),
            };
            let shuffle = _mm256_setr_epi32(1, 2, 3, 4, 5, 6, 7, 7);
            while i < n - (4 * 7 + 1) {
                let curr0 = _mm256_loadu_si256((&a[i..]).as_ptr() as *const _);
                let curr1 = _mm256_loadu_si256((&a[i + 7..]).as_ptr() as *const _);
                let curr2 = _mm256_loadu_si256((&a[i + 7 * 2..]).as_ptr() as *const _);
                let curr3 = _mm256_loadu_si256((&a[i + 7 * 3..]).as_ptr() as *const _);

                let next0 = _mm256_permutevar8x32_epi32(curr0, shuffle);
                let next1 = _mm256_permutevar8x32_epi32(curr1, shuffle);
                let next2 = _mm256_permutevar8x32_epi32(curr2, shuffle);
                let next3 = _mm256_permutevar8x32_epi32(curr3, shuffle);

                let mask0 = compare(curr0, next0);
                let mask1 = compare(curr1, next1);
                let mask2 = compare(curr2, next2);
                let mask3 = compare(curr3, next3);

                let mask =
                    _mm256_or_si256(mask0, _mm256_or_si256(mask1, _mm256_or_si256(mask2, mask3)));

                if _mm256_testz_si256(mask, mask) != 1 {
                    return false;
                }
                i += 7 * 4;
            }
        }
    }

    is_sorted_scalar(a, n, i, t)
}

#[cfg(target_feature = "avx2")]
#[cfg(test)]
mod tests {

    #[cfg(all(
        target_feature = "avx2",
        any(target_arch = "x86_64", target_arch = "x86")
    ))]
    #[test]
    fn test_works() {
        use crate::simd::avx2::*;
        use crate::simd::Trend;
        let mut nums = vec![0, 1, 2, 3, 4, 5, 6, 7];
        assert!(
            is_sorted_avx2(&nums, Trend::Ascending),
            "vector is ascending"
        );
        assert!(
            !is_sorted_avx2(&nums, Trend::Descending),
            "vector is not descending"
        );
        nums.reverse();
        assert!(
            is_sorted_avx2(&nums, Trend::Descending),
            "vector is descending"
        );
        assert!(
            !is_sorted_avx2(&nums, Trend::Ascending),
            "vector is not ascending"
        );

        let nums = vec![1, 0, 3, 2, 4, 5, 6, 7];
        assert!(
            !is_sorted_avx2(&nums, Trend::Ascending),
            "vector is not sorted"
        );
        assert!(
            !is_sorted_avx2(&nums, Trend::Descending),
            "vector is not sorted"
        );
        let mut nums = (0i32..64).into_iter().collect::<Vec<_>>();
        assert!(
            is_sorted_avx2_unroll4(&nums, Trend::Ascending),
            "64 vector is sorted"
        );

        nums.reverse();
        assert!(
            is_sorted_avx2_unroll4(&nums, Trend::Descending),
            "64 vector is sorted"
        );

        let nums = vec![1i32; 8];
        assert!(
            is_sorted_avx2(&nums, Trend::Ascending),
            "vector is ascending"
        );
        let nums = vec![1, 2, 2, 2, 2, 2, 2, 3];
        assert!(
            is_sorted_avx2(&nums, Trend::Ascending),
            "vector is ascending"
        );

        let mut nums = (0u32..8).into_iter().collect::<Vec<_>>();
        assert!(is_sorted_avx2(&nums, Trend::Ascending));
        nums.reverse();
        assert!(is_sorted_avx2(&nums, Trend::Descending));
        nums[6] = u32::MAX;
        nums[7] = u32::MIN;
        assert!(!is_sorted_avx2(&nums, Trend::Ascending));
        assert!(!is_sorted_avx2(&nums, Trend::Descending));

        let mut nums = (0u32..128).into_iter().collect::<Vec<_>>();
        assert!(is_sorted_avx2_unroll4(&nums, Trend::Ascending));
        nums.reverse();
        assert!(is_sorted_avx2_unroll4(&nums, Trend::Descending));

        nums[126] = u32::MAX;
        nums[127] = u32::MIN;
        assert!(!is_sorted_avx2_unroll4(&nums, Trend::Ascending));
        assert!(!is_sorted_avx2_unroll4(&nums, Trend::Descending));
    }
}
