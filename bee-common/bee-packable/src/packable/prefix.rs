// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Types and utilities used to pack and unpack dynamic sequences of values with restricted length prefixes.

extern crate alloc;

use crate::{
    error::{UnpackError, UnpackErrorExt},
    packable::bounded::{
        Bounded, BoundedU16, BoundedU32, BoundedU64, BoundedU8, InvalidBoundedU16, InvalidBoundedU32,
        InvalidBoundedU64, InvalidBoundedU8,
    },
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};

use alloc::{boxed::Box, vec::Vec};
use core::{
    convert::Infallible,
    fmt::{self, Display},
    marker::PhantomData,
};

/// Semantic error raised when converting a [`Vec`] into a [`VecPrefix`] or `Box<[_]>` into a [`BoxedSlicePrefix`].
#[derive(Debug)]
pub enum TryIntoPrefixError<E> {
    /// The prefix length was truncated.
    Truncated(usize),
    /// The prefix length is invalid.
    Invalid(E),
}

impl<E> From<E> for TryIntoPrefixError<E> {
    fn from(err: E) -> Self {
        Self::Invalid(err)
    }
}

impl<E: Display> Display for TryIntoPrefixError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Truncated(len) => write!(f, "length of `{}` was truncated", len),
            Self::Invalid(err) => err.fmt(f),
        }
    }
}

/// Semantic error raised while unpacking a dynamically-sized sequences that use a type different than `usize` for their
/// length-prefix.
#[derive(Debug)]
pub enum UnpackPrefixError<T, E> {
    /// Semantic error raised while unpacking an element of the sequence.
    /// Typically this is [`Packable::UnpackError`](crate::Packable::UnpackError).
    Packable(T),
    /// Semantic error raised when the length prefix cannot be unpacked.
    Prefix(E),
}

impl<E> UnpackPrefixError<Infallible, E> {
    /// Projects the value to the [`Prefix`](UnpackPrefixError::Prefix) variant.
    pub fn into_prefix(self) -> E {
        match self {
            Self::Packable(err) => match err {},
            Self::Prefix(err) => err,
        }
    }
}

impl<T, E> UnpackPrefixError<T, E> {
    /// Returns the contained [`Packable`](UnpackPrefixError::Packable) value or computes it from a closure.
    pub fn unwrap_packable_or_else<V: Into<T>>(self, f: impl FnOnce(E) -> V) -> T {
        match self {
            Self::Packable(err) => err,
            Self::Prefix(err) => f(err).into(),
        }
    }
}

impl<T, E> From<T> for UnpackPrefixError<T, E> {
    fn from(err: T) -> Self {
        Self::Packable(err)
    }
}

/// Wrapper type for [`Vec<T>`] with a length prefix.
/// The [`Vec<T>`]'s prefix bounds are provided by `B`, where `B` is a [`Bounded`] type.
/// The prefix type is the `Bounds` type associated with `B`.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct VecPrefix<T, B: Bounded> {
    inner: Vec<T>,
    bounded: PhantomData<B>,
}

impl<T, B: Bounded> Default for VecPrefix<T, B> {
    fn default() -> Self {
        Self {
            inner: Vec::new(),
            bounded: PhantomData,
        }
    }
}

macro_rules! impl_vec_prefix {
    ($ty:ty, $bounded:ty, $err:ty, $unpack_err:ty, $map_err:expr, $($generics:tt)*) => {
        impl<T, $($generics)*> TryFrom<Vec<T>> for VecPrefix<T, $bounded> {
            type Error = TryIntoPrefixError<$err>;

            fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
                let len = vec.len();
                let _ =
                    <$bounded>::try_from(<$ty>::try_from(len).map_err(|_| TryIntoPrefixError::Truncated(len))?)
                        .map_err(TryIntoPrefixError::Invalid)?;

                Ok(Self {
                    inner: vec,
                    bounded: PhantomData,
                })
            }
        }

        impl<'a, T, $($generics)*> TryFrom<&'a Vec<T>> for &'a VecPrefix<T, $bounded> {
            type Error = TryIntoPrefixError<$err>;

            fn try_from(vec: &Vec<T>) -> Result<Self, Self::Error> {
                let len = vec.len();
                let _ =
                    <$bounded>::try_from(<$ty>::try_from(len).map_err(|_| TryIntoPrefixError::Truncated(len))?)
                        .map_err(TryIntoPrefixError::Invalid)?;

                // SAFETY: `Vec<T>` and `VecPrefix<T, B>` have the same layout.
                Ok(unsafe { &*(vec as *const Vec<T> as *const VecPrefix<T, $bounded>) })
            }
        }

        /// We cannot provide a [`From`] implementation because [`Vec`] is not from this crate.
        #[allow(clippy::from_over_into)]
        impl<T, $($generics)*> Into<Vec<T>> for VecPrefix<T, $bounded> {
            fn into(self) -> Vec<T> {
                self.inner
            }
        }

        impl<T, $($generics)*> core::ops::Deref for VecPrefix<T, $bounded> {
            type Target = Vec<T>;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl<T: Packable, $($generics)*> Packable for VecPrefix<T, $bounded> {
            type UnpackError = $unpack_err;

            fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
                // The length of any dynamically-sized sequence must be prefixed.
                // This unwrap is fine, since we have already validated the length in `try_from`.
                <$ty>::try_from(self.len()).unwrap().pack(packer)?;

                for item in self.iter() {
                    item.pack(packer)?;
                }

                Ok(())
            }

            fn unpack<U: Unpacker, const VERIFY: bool>(
                unpacker: &mut U,
            ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
                // The length of any dynamically-sized sequence must be prefixed.
                let len: $ty = <$bounded>::unpack::<_, VERIFY>(unpacker)
                    .map_packable_err($map_err)?
                    .into();

                let mut inner = Vec::with_capacity(len as usize);

                for _ in 0..len {
                    let item = T::unpack::<_, VERIFY>(unpacker).coerce()?;
                    inner.push(item);
                }

                Ok(VecPrefix {
                    inner,
                    bounded: PhantomData,
                })
            }
        }
    };
}

impl_vec_prefix!(u8, u8, Infallible, T::UnpackError, |err| match err {},);
impl_vec_prefix!(u16, u16, Infallible, T::UnpackError, |err| match err {},);
impl_vec_prefix!(u32, u32, Infallible, T::UnpackError, |err| match err {},);
impl_vec_prefix!(u64, u64, Infallible, T::UnpackError, |err| match err {},);

impl_vec_prefix!(u8, BoundedU8<MIN, MAX>, InvalidBoundedU8<MIN, MAX>, UnpackPrefixError<T::UnpackError, InvalidBoundedU8<MIN, MAX>>, UnpackPrefixError::Prefix, const MIN: u8, const MAX: u8);
impl_vec_prefix!(u16, BoundedU16<MIN, MAX>, InvalidBoundedU16<MIN, MAX>, UnpackPrefixError<T::UnpackError, InvalidBoundedU16<MIN, MAX>>, UnpackPrefixError::Prefix, const MIN: u16, const MAX: u16);
impl_vec_prefix!(u32, BoundedU32<MIN, MAX>, InvalidBoundedU32<MIN, MAX>, UnpackPrefixError<T::UnpackError, InvalidBoundedU32<MIN, MAX>>, UnpackPrefixError::Prefix, const MIN: u32, const MAX: u32);
impl_vec_prefix!(u64, BoundedU64<MIN, MAX>, InvalidBoundedU64<MIN, MAX>, UnpackPrefixError<T::UnpackError, InvalidBoundedU64<MIN, MAX>>, UnpackPrefixError::Prefix, const MIN: u64, const MAX: u64);

/// Wrapper type for `Box<[T]>` with a length prefix.
/// The boxed slice's prefix bounds are provided by `B`, where `B` is a [`Bounded`] type.
/// The prefix type is the `Bounds` type associated with `B`.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct BoxedSlicePrefix<T, B: Bounded> {
    inner: Box<[T]>,
    bounded: PhantomData<B>,
}

impl<T, B: Bounded> Default for BoxedSlicePrefix<T, B> {
    fn default() -> Self {
        Self {
            inner: Box::new([]),
            bounded: PhantomData,
        }
    }
}

macro_rules! impl_boxed_slice_prefix {
    ($ty:ty, $bounded:ty, $err:ty, $unpack_err:ty, $map_err:expr, $($generics:tt)*) => {
        impl<T, $($generics)*> TryFrom<Box<[T]>> for BoxedSlicePrefix<T, $bounded> {
            type Error = TryIntoPrefixError<$err>;

            fn try_from(boxed_slice: Box<[T]>) -> Result<Self, Self::Error> {
                let len = boxed_slice.len();
                let _ =
                    <$bounded>::try_from(<$ty>::try_from(len).map_err(|_| TryIntoPrefixError::Truncated(len))?)
                        .map_err(TryIntoPrefixError::Invalid)?;

                Ok(Self {
                    inner: boxed_slice,
                    bounded: PhantomData,
                })
            }
        }

        impl<'a, T, $($generics)*> TryFrom<&'a Box<[T]>>
            for &'a BoxedSlicePrefix<T, $bounded>
        {
            type Error = TryIntoPrefixError<$err>;

            fn try_from(boxed_slice: &Box<[T]>) -> Result<Self, Self::Error> {
                let len = boxed_slice.len();
                let _ =
                    <$bounded>::try_from(<$ty>::try_from(len).map_err(|_| TryIntoPrefixError::Truncated(len))?)
                        .map_err(TryIntoPrefixError::Invalid)?;

                // SAFETY: `Box<[T]>` and `BoxedSlicePrefix<T, B>` have the same layout.
                Ok(unsafe { &*(boxed_slice as *const Box<[T]> as *const BoxedSlicePrefix<T, $bounded>) })
            }
        }

        /// We cannot provide a [`From`] implementation because [`Vec`] is not from this crate.
        #[allow(clippy::from_over_into)]
        impl<T, $($generics)*> Into<Box<[T]>> for BoxedSlicePrefix<T, $bounded> {
            fn into(self) -> Box<[T]> {
                self.inner
            }
        }

        impl<T, $($generics)*> core::ops::Deref for BoxedSlicePrefix<T, $bounded> {
            type Target = Box<[T]>;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl<T: Packable, $($generics)*> Packable for BoxedSlicePrefix<T, $bounded> {
            type UnpackError = $unpack_err;

            fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
                // The length of any dynamically-sized sequence must be prefixed.
                // This unwrap is fine, since we have already validated the length in `try_from`.
                <$ty>::try_from(self.len()).unwrap().pack(packer)?;

                for item in self.iter() {
                    item.pack(packer)?;
                }

                Ok(())
            }

            fn unpack<U: Unpacker, const VERIFY: bool>(
                unpacker: &mut U,
            ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
                // The length of any dynamically-sized sequence must be prefixed.
                let len: $ty = <$bounded>::unpack::<_, VERIFY>(unpacker)
                    .map_packable_err($map_err)?
                    .into();

                let mut inner = Vec::with_capacity(len as usize);

                for _ in 0..len {
                    let item = T::unpack::<_, VERIFY>(unpacker).coerce()?;
                    inner.push(item);
                }

                Ok(BoxedSlicePrefix {
                    inner: inner.into_boxed_slice(),
                    bounded: PhantomData,
                })
            }
        }
    };
}

impl_boxed_slice_prefix!(u8, u8, Infallible, T::UnpackError, |err| match err {},);
impl_boxed_slice_prefix!(u16, u16, Infallible, T::UnpackError, |err| match err {},);
impl_boxed_slice_prefix!(u32, u32, Infallible, T::UnpackError, |err| match err {},);
impl_boxed_slice_prefix!(u64, u64, Infallible, T::UnpackError, |err| match err {},);

impl_boxed_slice_prefix!(u8, BoundedU8<MIN, MAX>, InvalidBoundedU8<MIN, MAX>, UnpackPrefixError<T::UnpackError, InvalidBoundedU8<MIN, MAX>>, UnpackPrefixError::Prefix, const MIN: u8, const MAX: u8);
impl_boxed_slice_prefix!(u16, BoundedU16<MIN, MAX>, InvalidBoundedU16<MIN, MAX>, UnpackPrefixError<T::UnpackError, InvalidBoundedU16<MIN, MAX>>, UnpackPrefixError::Prefix, const MIN: u16, const MAX: u16);
impl_boxed_slice_prefix!(u32, BoundedU32<MIN, MAX>, InvalidBoundedU32<MIN, MAX>, UnpackPrefixError<T::UnpackError, InvalidBoundedU32<MIN, MAX>>, UnpackPrefixError::Prefix, const MIN: u32, const MAX: u32);
impl_boxed_slice_prefix!(u64, BoundedU64<MIN, MAX>, InvalidBoundedU64<MIN, MAX>, UnpackPrefixError<T::UnpackError, InvalidBoundedU64<MIN, MAX>>, UnpackPrefixError::Prefix, const MIN: u64, const MAX: u64);