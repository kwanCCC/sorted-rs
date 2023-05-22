/// sse part
#[cfg(feature = "use-avx2")]
pub use avx2::is_sorted_avx2 as is_sorted;
#[cfg(feature = "use-avx2")]
pub use avx2::is_sorted_avx2_unroll4 as is_sorted_unroll4;
/// avx part
#[cfg(feature = "use-sse")]
pub use sse::is_sorted_sse as is_sorted;
#[cfg(feature = "use-sse")]
pub use sse::is_sorted_sse_unroll4 as is_sorted_unroll4;

mod avx2;
mod sse;

/// marker trait for inputs limitation
pub trait SinglePrecision: Sized {}
/// u32 is valid
impl SinglePrecision for u32 {}
/// i32 is valid
impl SinglePrecision for i32 {}

/// `Trend` is used to describe the trending of input which u want to check
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Trend {
    Ascending,
    Descending,
}

/// `is_sorted_scalar` a generic impls `is_sorted`
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
