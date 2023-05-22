#[cfg(feature = "use-avx2")]
pub use avx2::is_sorted_avx2 as is_sorted;
#[cfg(feature = "use-avx2")]
pub use avx2::is_sorted_avx2_unroll4 as is_sorted_unroll4;
#[cfg(feature = "use-sse")]
pub use sse::is_sorted_sse as is_sorted;
#[cfg(feature = "use-sse")]
pub use sse::is_sorted_sse_unroll4 as is_sorted_unroll4;

mod avx2;
mod sse;

pub trait SinglePrecision: Sized {}
impl SinglePrecision for u32 {}
impl SinglePrecision for i32 {}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Trend {
    Ascending,
    Descending,
}

#[inline]
fn is_sorted_scalar<N: num::Integer, T: AsRef<[N]>>(
    a: T,
    len: usize,
    mut index: usize,
    trend: Trend,
) -> bool {
    let compare = match trend {
        Trend::Ascending => |a: &N, b: &N| a > b,
        Trend::Descending => |a: &N, b: &N| a < b,
    };
    let a = a.as_ref();
    while index + 1 < len {
        if compare(&a[index], &a[index + 1]) {
            return false;
        }
        index += 1;
    }
    true
}
