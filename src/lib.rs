#![allow(dead_code)]

pub use simd::Trend;

use crate::simd::SinglePrecision;

mod simd;

///
/// example:
///     let nums = vec![1, 2, 3, 4, 5, ....];
///     is_sorted(&nums, Trend::Ascending);
///     let nums = vec![9, 8, 7, 6, 5, ....];
///     is_sorted(&nums, Trend:Descending);
///
/// is_sorted check sequence is sorted or not but it doesn't check the length of input.
/// It's better to avoid call the function when the length of input is 1 or 0;

pub fn is_sorted<T: num::Integer + SinglePrecision>(a: &[T], t: Trend) -> bool {
    let a = a.as_ref();
    let n = a.len();
    #[cfg(feature = "use-sse")]
    {
        if n < SSE_WORD {
            return simd::is_sorted(a, t);
        }
    }
    #[cfg(feature = "use-avx2")]
    {
        if n < AVX2_WORD {
            return simd::is_sorted(&a, t);
        }
    }
    #[cfg(feature = "use-sse")]
    {
        if n >= SSE_WORD {
            return simd::is_sorted_unroll4(&a, t);
        }
    }
    #[cfg(feature = "use-avx2")]
    {
        if n >= AVX2_WORD {
            return simd::is_sorted_unroll4(&a, t);
        }
    }
    false
}

const WORD: usize = 8;
const SSE_WORD: usize = 4 * (4 + 1);
const AVX2_WORD: usize = 4 * 7 + 1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_works() {
        for i in 1..1024 {
            let mut a = (0..i).into_iter().collect::<Vec<_>>();
            assert!(is_sorted(&a, Trend::Ascending));
            a.reverse();
            assert!(is_sorted(&a, Trend::Descending));
            a.fill(1);
            assert!(is_sorted(&a, Trend::Ascending));
            assert!(is_sorted(&a, Trend::Descending));
        }

        for i in 6..1024 {
            let a = (0..i).into_iter().collect::<Vec<_>>();
            let mut reversed: Vec<i32> = Vec::with_capacity(a.len());
            for cur_data in a.chunks(3) {
                reversed.extend(cur_data.iter().rev());
            }

            assert!(!is_sorted(&reversed, Trend::Ascending), "{:?}", reversed);
            assert!(!is_sorted(&reversed, Trend::Descending), "{:?}", reversed);
        }

        for i in 1u32..1024 {
            let mut a = (0..i).into_iter().collect::<Vec<_>>();
            assert!(is_sorted(&a, Trend::Ascending));
            a.reverse();
            assert!(is_sorted(&a, Trend::Descending));
            a.fill(1);
            assert!(is_sorted(&a, Trend::Ascending));
            assert!(is_sorted(&a, Trend::Descending));
        }
    }
}
