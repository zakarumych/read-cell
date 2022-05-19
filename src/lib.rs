//!
//! Provides read-only counterpart to standard [`Cell`] type.
//! Unlike [`Cell`], [`ReadCell`] cannot be used to mutate inner value, just like [`&T`],
//! but similar to [`Cell`] it cannot be used to get [`&T`] to the inner value.
//!
//! While [`&Cell<T>`] references and [`&T`] references to the same value cannot coexist,
//! [`&ReadCell<T>`] reference and [`&Cell<T>`] reference to the same value can coexist.
//! As well as [`&ReadCell<T>`] reference and [`&T`] reference to the same value can coexist.
//!
//! [`&Cell<T>`]: `Cell`
//! [`&ReadCell<T>`]: `ReadCell`
//! [`&T`]: `reference`

#![no_std]

use core::{
    cell::{Cell, UnsafeCell},
    cmp::Ordering,
};

/// A possible mutable memory location.
/// It provides only non-mutating subset of [`Cell`] API.
/// This allows [`&ReadCell<T>`] share value with [`&Cell<T>`] and [`&T`] alike.
///
/// [`&ReadCell<T>`]: `ReadCell`
/// [`&Cell<T>`]: `Cell`
/// [`&T`]: `reference`
///
/// # Example
///
/// ```
/// use std::cell::Cell;
/// use read_cell::ReadCell;
///
/// struct SomeStruct {
///     regular_field: u8,
///     special_field: Cell<u8>,
/// }
///
/// let my_struct = SomeStruct {
///     regular_field: 0,
///     special_field: Cell::new(1),
/// };
///
/// let new_value = 100;
///
/// let regular_field_read = ReadCell::from_ref(&my_struct.regular_field);
/// let special_field_read = ReadCell::from_cell(&my_struct.special_field);
///
/// assert_eq!(regular_field_read.get(), 0);
/// assert_eq!(special_field_read.get(), 1);
///
/// my_struct.special_field.set(new_value);
/// assert_eq!(special_field_read.get(), new_value);
/// ```
#[repr(transparent)]
pub struct ReadCell<T: ?Sized> {
    value: UnsafeCell<T>,
}

impl<T: Copy> Clone for ReadCell<T> {
    #[inline]
    fn clone(&self) -> ReadCell<T> {
        ReadCell::new(self.get())
    }
}

impl<T: Default> Default for ReadCell<T> {
    /// Creates a `ReadCell<T>`, with the `Default` value for T.
    #[inline]
    fn default() -> ReadCell<T> {
        ReadCell::new(Default::default())
    }
}

impl<T: PartialEq + Copy> PartialEq for ReadCell<T> {
    #[inline]
    fn eq(&self, other: &ReadCell<T>) -> bool {
        self.get() == other.get()
    }
}

impl<T: Eq + Copy> Eq for ReadCell<T> {}

impl<T: PartialOrd + Copy> PartialOrd for ReadCell<T> {
    #[inline]
    fn partial_cmp(&self, other: &ReadCell<T>) -> Option<Ordering> {
        self.get().partial_cmp(&other.get())
    }

    #[inline]
    fn lt(&self, other: &ReadCell<T>) -> bool {
        self.get() < other.get()
    }

    #[inline]
    fn le(&self, other: &ReadCell<T>) -> bool {
        self.get() <= other.get()
    }

    #[inline]
    fn gt(&self, other: &ReadCell<T>) -> bool {
        self.get() > other.get()
    }

    #[inline]
    fn ge(&self, other: &ReadCell<T>) -> bool {
        self.get() >= other.get()
    }
}

impl<T: Ord + Copy> Ord for ReadCell<T> {
    #[inline]
    fn cmp(&self, other: &ReadCell<T>) -> Ordering {
        self.get().cmp(&other.get())
    }
}

impl<T> From<T> for ReadCell<T> {
    /// Creates a new `ReadCell<T>` containing the given value.
    fn from(t: T) -> ReadCell<T> {
        ReadCell::new(t)
    }
}

impl<T> ReadCell<T> {
    /// Creates a new `ReadCell` containing the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use read_cell::ReadCell;
    ///
    /// let c = ReadCell::new(5);
    /// ```
    #[inline]
    pub const fn new(value: T) -> ReadCell<T> {
        ReadCell {
            value: UnsafeCell::new(value),
        }
    }

    /// Unwraps the value.
    ///
    /// # Examples
    ///
    /// ```
    /// use read_cell::ReadCell;
    ///
    /// let c = ReadCell::new(5);
    /// let five = c.into_inner();
    ///
    /// assert_eq!(five, 5);
    /// ```
    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }
}

impl<T: Copy> ReadCell<T> {
    /// Returns a copy of the contained value.
    ///
    /// # Examples
    ///
    /// ```
    /// use read_cell::ReadCell;
    ///
    /// let c = ReadCell::new(5);
    ///
    /// let five = c.get();
    /// ```
    #[inline]
    pub fn get(&self) -> T {
        // SAFETY: This can cause data races if called from a separate thread,
        // but `ReadCell` is `!Sync` so this won't happen.
        unsafe { *self.value.get() }
    }
}

impl<T: ?Sized> ReadCell<T> {
    /// Returns a raw pointer to the underlying data in this cell.
    ///
    /// # Examples
    ///
    /// ```
    /// use read_cell::ReadCell;
    ///
    /// let c = ReadCell::new(5);
    ///
    /// let ptr = c.as_ptr();
    /// ```
    #[inline]
    pub const fn as_ptr(&self) -> *mut T {
        self.value.get()
    }

    /// Returns a mutable reference to the underlying data.
    ///
    /// This call borrows `ReadCell` mutably (at compile-time) which guarantees
    /// that we possess the only reference.
    ///
    /// However be cautious: this method expects `self` to be mutable, which is
    /// generally not the case when using a [`ReadCell`]. If you require interior
    /// mutability by reference, consider using [`RefCell`] which provides
    /// run-time checked mutable borrows through its [`borrow_mut`] method.
    ///
    /// [`RefCell`]: `core::cell::RefCell`
    /// [`borrow_mut`]: `core::cell::RefCell::borrow_mut`
    ///
    /// # Examples
    ///
    /// ```
    /// use read_cell::ReadCell;
    ///
    /// let mut c = ReadCell::new(5);
    /// *c.get_mut() += 1;
    ///
    /// assert_eq!(c.get(), 6);
    /// ```
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        self.value.get_mut()
    }

    /// Returns a `&ReadCell<T>` from a `&T`
    ///
    /// # Examples
    ///
    /// ```
    /// use read_cell::ReadCell;
    ///
    /// let slice: &[i32] = &[1, 2, 3];
    /// let cell_slice: &ReadCell<[i32]> = ReadCell::from_ref(slice);
    /// let slice_cell: &[ReadCell<i32>] = cell_slice.as_slice_of_cells();
    ///
    /// assert_eq!(slice_cell.len(), 3);
    /// ```
    #[inline]
    pub fn from_ref(t: &T) -> &ReadCell<T> {
        // SAFETY: `&ReadCell<T>` disallows mutations.
        unsafe { &*(t as *const T as *const ReadCell<T>) }
    }

    /// Returns a `&ReadCell<T>` from a `&Cell<T>`
    ///
    /// # Examples
    ///
    /// ```
    /// use std::cell::Cell;
    /// use read_cell::ReadCell;
    ///
    /// let slice: &Cell<[i32]> = &Cell::new([1, 2, 3]);
    /// let cell_slice: &ReadCell<[i32]> = ReadCell::from_cell(slice);
    /// let slice_cell: &[ReadCell<i32>] = cell_slice.as_slice_of_cells();
    ///
    /// assert_eq!(slice_cell.len(), 3);
    /// ```
    #[inline]
    pub fn from_cell(t: &Cell<T>) -> &ReadCell<T> {
        // SAFETY: `&ReadCell<T>` is more restricted than `Cell`.
        unsafe { &*(t.as_ptr() as *const ReadCell<T>) }
    }
}

impl<T> ReadCell<[T]> {
    /// Returns a `&[ReadCell<T>]` from a `&ReadCell<[T]>`
    ///
    /// # Examples
    ///
    /// ```
    /// use read_cell::ReadCell;
    ///
    /// let slice: &[i32] = &[1, 2, 3];
    /// let cell_slice: &ReadCell<[i32]> = ReadCell::from_ref(slice);
    /// let slice_cell: &[ReadCell<i32>] = cell_slice.as_slice_of_cells();
    ///
    /// assert_eq!(slice_cell.len(), 3);
    /// ```
    pub fn as_slice_of_cells(&self) -> &[ReadCell<T>] {
        // SAFETY: `ReadCell<T>` has the same memory layout as `T`.
        unsafe { &*(self as *const ReadCell<[T]> as *const [ReadCell<T>]) }
    }
}

impl<T, const N: usize> ReadCell<[T; N]> {
    /// Returns a `&[ReadCell<T>; N]` from a `&ReadCell<[T; N]>`
    ///
    /// # Examples
    ///
    /// ```
    /// use read_cell::ReadCell;
    ///
    /// let mut array: [i32; 3] = [1, 2, 3];
    /// let cell_array: &ReadCell<[i32; 3]> = ReadCell::from_ref(&array);
    /// let array_cell: &[ReadCell<i32>; 3] = cell_array.as_array_of_cells();
    /// ```
    pub fn as_array_of_cells(&self) -> &[ReadCell<T>; N] {
        // SAFETY: `ReadCell<T>` has the same memory layout as `T`.
        unsafe { &*(self as *const ReadCell<[T; N]> as *const [ReadCell<T>; N]) }
    }
}
