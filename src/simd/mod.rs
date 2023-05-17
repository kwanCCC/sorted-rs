#[cfg(feature = "use-avx2")]
use avx2::is_sorted_avx2 as is_sorted;
#[cfg(feature = "use-avx2")]
use avx2::is_sorted_avx2_unroll4 as is_sorted_unroll4;
#[cfg(feature = "use-sse")]
use sse::is_sorted_sse as is_sorted;
#[cfg(feature = "use-sse")]
use sse::is_sorted_sse_unroll4 as is_sorted_unroll4;

mod avx2;
mod sse;

pub fn is_sort<T: AsRef<[i32]>>(a: T, t: Trend) -> bool {
    is_sorted(a, t)
}

pub fn is_sort_unroll4<T: AsRef<[i32]>>(a: T, t: Trend) -> bool {
    is_sorted_unroll4(a, t)
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Trend {
    Ascending,
    Descending,
}

#[inline]
fn is_sorted_scalar<T: AsRef<[i32]>>(a: T, len: usize, mut index: usize, trend: Trend) -> bool {
    let compare = match trend {
        Trend::Ascending => |a: i32, b: i32| a > b,
        Trend::Descending => |a: i32, b: i32| a < b,
    };
    let a = a.as_ref();
    while index + 1 < len {
        if compare(a[index], a[index + 1]) {
            return false;
        }
        index += 1;
    }
    true
}
