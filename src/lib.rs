/*!

 */

#![cfg_attr(not(test), no_std)]

mod ptr32;
mod ptr64;

pub use self::ptr32::*;
pub use self::ptr64::*;

#[cfg(target_pointer_width = "32")]
pub use IntPtr32 as IntPtr;

#[cfg(target_pointer_width = "64")]
pub use IntPtr64 as IntPtr;

impl<T: ?Sized> From<IntPtr32<T>> for IntPtr64<T> {
	#[inline(always)]
	fn from(ptr: IntPtr32<T>) -> IntPtr64<T> {
		IntPtr64::from(ptr.into_raw() as u64)
	}
}
