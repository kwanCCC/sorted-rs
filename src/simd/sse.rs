#![allow(dead_code)]

use crate::simd::{is_sorted_scalar, Trend};

#[cfg(all(
    target_feature = "sse",
    any(target_arch = "x86_64", target_arch = "x86")
))]
pub(crate) fn is_sorted_sse<T: AsRef<[i32]>>(a: T, trend: Trend) -> bool {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    let a = a.as_ref();
    let l = a.len();
    let mut i = 0usize;
    if l >= 8 {
        unsafe {
            let compare = match trend {
                Trend::Ascending => _mm_cmpgt_epi32,
                Trend::Descending => _mm_cmplt_epi32,
            };

            let astar = &a[0..4];
            let mut chunk0 = _mm_loadu_si128(astar.as_ptr() as *const _);
            while i < l - 4 {
                let bstar = &a[i + 4..];
                let chunk1 = _mm_loadu_si128(bstar.as_ptr() as *const _);
                let current = chunk0;
                let next = _mm_alignr_epi8::<4>(chunk1, chunk0);
                let mask = compare(current, next);

                if _mm_test_all_zeros(mask, mask) != 1 {
                    return false;
                }
                chunk0 = chunk1;
                i += 4;
            }
        }
    }
    is_sorted_scalar(a, l, i, trend)
}

#[cfg(all(
    target_feature = "sse",
    any(target_arch = "x86_64", target_arch = "x86")
))]
pub(crate) fn is_sorted_sse_unroll4<T: AsRef<[i32]>>(a: T, trend: Trend) -> bool {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    let a = a.as_ref();
    let l = a.len();
    let mut i = 0usize;
    if l >= 4 * (4 + 1) {
        unsafe {
            let compare = match trend {
                Trend::Ascending => _mm_cmpgt_epi32,
                Trend::Descending => _mm_cmplt_epi32,
            };

            let mut chunk0 = _mm_loadu_si128((&a[0..]).as_ptr() as *const _);
            while i < l - 4 * 4 {
                let chunk1 = _mm_loadu_si128((&a[i + 1 * 4..]).as_ptr() as *const _);
                let chunk2 = _mm_loadu_si128((&a[i + 2 * 4..]).as_ptr() as *const _);
                let chunk3 = _mm_loadu_si128((&a[i + 3 * 4..]).as_ptr() as *const _);
                let chunk4 = _mm_loadu_si128((&a[i + 4 * 4..]).as_ptr() as *const _);

                let next0 = _mm_alignr_epi8::<4>(chunk1, chunk0);
                let next1 = _mm_alignr_epi8::<4>(chunk2, chunk1);
                let next2 = _mm_alignr_epi8::<4>(chunk3, chunk2);
                let next3 = _mm_alignr_epi8::<4>(chunk4, chunk3);

                let mask0 = compare(chunk0, next0);
                let mask1 = compare(chunk1, next1);
                let mask2 = compare(chunk2, next2);
                let mask3 = compare(chunk3, next3);

                let mask = _mm_or_si128(mask0, _mm_or_si128(mask1, _mm_or_si128(mask2, mask3)));

                if _mm_test_all_zeros(mask, mask) != 1 {
                    return false;
                }
                chunk0 = chunk4;
                i += 4 * 4
            }
        }
    }
    is_sorted_scalar(a, l, i, trend)
}

#[cfg(all(
    target_feature = "sse",
    any(target_arch = "x86_64", target_arch = "x86")
))]
fn is_sorted_sse_generic<T: AsRef<[i32]>>(a: T, trend: Trend) -> bool {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;
    let a = a.as_ref();
    let len = a.len();
    let mut i = 0usize;

    if len > 4 {
        unsafe {
            let compare = match trend {
                Trend::Ascending => _mm_cmpgt_epi32,
                Trend::Descending => _mm_cmplt_epi32,
            };

            while i < len - 4 {
                let curr = _mm_loadu_si128((&a[i..]).as_ptr() as *const _);
                let next = _mm_loadu_si128((&a[i + 1..]).as_ptr() as *const _);
                let mask = compare(curr, next);
                if _mm_test_all_zeros(mask, mask) != 1 {
                    return false;
                }
                i += 4;
            }
        }
    }

    is_sorted_scalar(a, len, i, trend)
}

#[cfg(all(
    target_feature = "sse",
    any(target_arch = "x86_64", target_arch = "x86")
))]
fn is_sorted_sse_generic_unroll4<T: AsRef<[i32]>>(a: T, trend: Trend) -> bool {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    let a = a.as_ref();
    let len = a.len();
    let mut i = 0usize;
    if len >= 4 * 4 {
        unsafe {
            let compare = match trend {
                Trend::Ascending => _mm_cmpgt_epi32,
                Trend::Descending => _mm_cmplt_epi32,
            };

            while i < len - 4 * 4 {
                let current0 = _mm_loadu_si128((&a[i..]).as_ptr() as *const _);
                let current1 = _mm_loadu_si128((&a[i + 1 * 4..]).as_ptr() as *const _);
                let current2 = _mm_loadu_si128((&a[i + 2 * 4..]).as_ptr() as *const _);
                let current3 = _mm_loadu_si128((&a[i + 3 * 4..]).as_ptr() as *const _);

                let next0 = _mm_loadu_si128((&a[i + 1..]).as_ptr() as *const _);
                let next1 = _mm_loadu_si128((&a[i + 1 + 1 * 4..]).as_ptr() as *const _);
                let next2 = _mm_loadu_si128((&a[i + 1 + 2 * 4..]).as_ptr() as *const _);
                let next3 = _mm_loadu_si128((&a[i + 1 + 3 * 4..]).as_ptr() as *const _);

                let mask0 = compare(current0, next0);
                let mask1 = compare(current1, next1);
                let mask2 = compare(current2, next2);
                let mask3 = compare(current3, next3);
                let mask = _mm_or_si128(mask0, _mm_or_si128(mask1, _mm_or_si128(mask2, mask3)));
                if _mm_test_all_zeros(mask, mask) != 1 {
                    return false;
                }
                i += 4 * 4;
            }
        }
    }
    is_sorted_scalar(a, len, i, trend)
}

#[cfg(test)]
mod tests {
    use crate::simd::sse::{
        is_sorted_sse, is_sorted_sse_generic, is_sorted_sse_generic_unroll4, is_sorted_sse_unroll4,
    };
    use crate::simd::Trend;

    #[test]
    fn works() {
        let mut nums = vec![0, 1, 2, 3, 4, 5, 6, 7];
        assert!(
            is_sorted_sse(&nums, Trend::Ascending),
            "vector is ascending"
        );
        assert!(
            !is_sorted_sse(&nums, Trend::Descending),
            "vector is not descending"
        );
        assert!(
            is_sorted_sse_generic(&nums, Trend::Ascending),
            "vector is ascending"
        );
        assert!(
            !is_sorted_sse_generic(&nums, Trend::Descending),
            "vector is not descending"
        );
        nums.reverse();
        assert!(
            is_sorted_sse(&nums, Trend::Descending),
            "vector is descending"
        );
        assert!(
            !is_sorted_sse(&nums, Trend::Ascending),
            "vector is not ascending"
        );
        assert!(
            is_sorted_sse_generic(&nums, Trend::Descending),
            "vector is descending"
        );
        assert!(
            !is_sorted_sse_generic(&nums, Trend::Ascending),
            "vector is not ascending"
        );

        let nums = vec![1, 0, 3, 2, 4, 5, 6, 7];
        assert!(
            !is_sorted_sse(&nums, Trend::Ascending),
            "vector is not sorted"
        );
        assert!(
            !is_sorted_sse(&nums, Trend::Descending),
            "vector is not sorted"
        );
        let mut nums = (0i32..64).into_iter().collect::<Vec<_>>();
        assert!(
            is_sorted_sse_unroll4(&nums, Trend::Ascending),
            "64 vector is sorted"
        );
        assert!(
            is_sorted_sse_generic_unroll4(&nums, Trend::Ascending),
            "64 vector is sorted"
        );

        nums.reverse();
        assert!(
            is_sorted_sse_unroll4(&nums, Trend::Descending),
            "64 vector is sorted"
        );
        assert!(
            is_sorted_sse_generic_unroll4(&nums, Trend::Descending),
            "64 vector is sorted"
        );

        let nums = vec![1i32; 8];
        assert!(
            is_sorted_sse(&nums, Trend::Ascending),
            "vector is ascending"
        );
        let nums = vec![1, 2, 2, 2, 2, 2, 2, 3];
        assert!(
            is_sorted_sse(&nums, Trend::Ascending),
            "vector is ascending"
        );

        let mut nums = (0i32..127).into_iter().collect::<Vec<_>>();
        nums[125] = i32::MAX;
        nums[126] = i32::MIN;
        assert!(
            !is_sorted_sse_unroll4(&nums, Trend::Ascending),
            "64 vector is sorted"
        );
    }
}
