#[cfg(all(
    target_feature = "sse",
    any(target_arch = "x86_64", target_arch = "x86")
))]
fn ascending<T: AsRef<[i32]>>(a: T) -> bool {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    let a = a.as_ref();
    let l = a.len();
    let mut i = 0usize;
    if l >= 8 {
        unsafe {
            let astar = &a[0..4];
            let mut chunk0 = _mm_loadu_si128(astar.as_ptr() as *const _);
            while i < l - 4 {
                let bstar = &a[i + 4..];
                let chunk1 = _mm_loadu_si128(bstar.as_ptr() as *const _);
                let current = chunk0;
                let next = _mm_alignr_epi8::<4>(chunk1, chunk0);
                let mask = _mm_cmpgt_epi32(current, next);
                if _mm_test_all_zeros(mask, mask) != 1 {
                    return false;
                }
                chunk0 = chunk1;
                i += 4;
            }
        }
        return true;
    }
    ascending_scalar(a)
}

#[cfg(all(
    target_feature = "sse",
    any(target_arch = "x86_64", target_arch = "x86")
))]
fn ascending_unroll4<T: AsRef<[i32]>>(a: T) -> bool {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    let a = a.as_ref();
    let l = a.len();
    let mut i = 0usize;
    if l >= 4 * (4 + 1) {
        unsafe {
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

                let mask0 = _mm_cmpgt_epi32(chunk0, next0);
                let mask1 = _mm_cmpgt_epi32(chunk1, next1);
                let mask2 = _mm_cmpgt_epi32(chunk2, next2);
                let mask3 = _mm_cmpgt_epi32(chunk3, next3);

                let mask = _mm_or_si128(mask0, _mm_or_si128(mask1, _mm_or_si128(mask2, mask3)));

                if _mm_test_all_zeros(mask, mask) != 1 {
                    return false;
                }
                chunk0 = chunk4;
                i += 4 * 4
            }
        }
        return true;
    }

    ascending_scalar(a)
}

#[inline]
fn ascending_scalar<T: AsRef<[i32]>>(a: T) -> bool {
    let a = a.as_ref();
    let len = a.as_ref().len();
    for i in 1..len {
        if a[i] < a[i - 1] {
            return false;
        }
    }
    true
}

#[cfg(all(
    target_feature = "sse",
    any(target_arch = "x86_64", target_arch = "x86")
))]
fn ascending_generic<T: AsRef<[i32]>>(a: T) -> bool {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;
    let a = a.as_ref();
    let len = a.len();
    let mut i = 0usize;

    if len > 4 {
        unsafe {
            while i < len - 4 {
                let curr = _mm_loadu_si128((&a[i..]).as_ptr() as *const _);
                let next = _mm_loadu_si128((&a[i + 1..]).as_ptr() as *const _);
                let mask = _mm_cmpgt_epi32(curr, next);
                if _mm_test_all_zeros(mask, mask) != 1 {
                    return false;
                }
                i += 4;
            }
        }
        return true;
    }

    ascending_scalar(a)
}

#[cfg(all(
    target_feature = "sse",
    any(target_arch = "x86_64", target_arch = "x86")
))]
fn ascending_generic_unroll4<T: AsRef<[i32]>>(a: T) -> bool {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    let a = a.as_ref();
    let len = a.len();
    let mut i = 0usize;
    if len >= 4 * 4 {
        unsafe {
            while i < len - 4 * 4 {
                let current0 = _mm_loadu_si128((&a[i..]).as_ptr() as *const _);
                let current1 = _mm_loadu_si128((&a[i + 1 * 4..]).as_ptr() as *const _);
                let current2 = _mm_loadu_si128((&a[i + 2 * 4..]).as_ptr() as *const _);
                let current3 = _mm_loadu_si128((&a[i + 3 * 4..]).as_ptr() as *const _);

                let next0 = _mm_loadu_si128((&a[i + 1..]).as_ptr() as *const _);
                let next1 = _mm_loadu_si128((&a[i + 1 + 1 * 4..]).as_ptr() as *const _);
                let next2 = _mm_loadu_si128((&a[i + 1 + 2 * 4..]).as_ptr() as *const _);
                let next3 = _mm_loadu_si128((&a[i + 1 + 3 * 4..]).as_ptr() as *const _);

                let mask0 = _mm_cmpgt_epi32(current0, next0);
                let mask1 = _mm_cmpgt_epi32(current1, next1);
                let mask2 = _mm_cmpgt_epi32(current2, next2);
                let mask3 = _mm_cmpgt_epi32(current3, next3);
                let mask = _mm_or_si128(mask0, _mm_or_si128(mask1, _mm_or_si128(mask2, mask3)));
                if _mm_test_all_zeros(mask, mask) != 1 {
                    return false;
                }
                i += 4 * 4;
            }
        }
        return true;
    }
    return ascending(a);
}

#[cfg(test)]
mod tests {
    use crate::simd::sse::{
        ascending, ascending_generic, ascending_generic_unroll4, ascending_unroll4,
    };

    #[test]
    fn works() {
        let nums = vec![0, 1, 2, 3, 4, 5, 6, 7];
        assert!(ascending(&nums), "vector is sorted");
        assert!(ascending_generic(&nums), "vector is sorted");
        let nums = vec![1, 0, 3, 2, 4, 5, 6, 7];
        assert!(!ascending(&nums), "vector is not sorted");
        let nums = (0i32..64).into_iter().collect::<Vec<_>>();
        assert!(ascending_unroll4(&nums), "64 vector is sorted");
        assert!(ascending_generic_unroll4(&nums), "64 vector is sorted");
    }
}
