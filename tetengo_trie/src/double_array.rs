/*!
 * A double array.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

mod double_array_builder;

use std::fmt::{self, Debug, Formatter};

use crate::storage::Storage;

/**
 * A result type.
 *
 * # Type Parameters
 * * `T` - A type.
 */
pub type Result<T> = anyhow::Result<T>;

/**
 * A double array error.
 */
#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum DoubleArrayError {
    /**
     * density_factor must be greater than 0.
     */
    #[error("density_factor must be greater than 0.")]
    InvalidDensityFactor,
}

/// The double array element type.
pub type DoubleArrayElement<'a> = (&'a str, i32);
/**
 * A building observer set.
 */
pub struct BuldingObserverSet {
    _adding: Box<dyn FnOnce(&DoubleArrayElement<'_>)>,
    _done: Box<dyn FnOnce()>,
}

impl BuldingObserverSet {
    /**
     * Creates a building observer set.
     *
     * # Parameters
     * * `adding` - An adding observer.
     * * `done` - A done observer.
     */
    pub fn new(adding: Box<dyn FnOnce(&DoubleArrayElement<'_>)>, done: Box<dyn FnOnce()>) -> Self {
        Self {
            _adding: adding,
            _done: done,
        }
    }
}

impl Debug for BuldingObserverSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("BuldingObserverSet")
            .field("adding", &"Box<dyn FnOnce(&DoubleArrayElement<'_>)>")
            .finish()
    }
}

/**
 * A double array.
 */
pub struct DoubleArray<V> {
    _storage: Box<dyn Storage<V>>,
    _root_base_check_index: usize,
}

impl<V> DoubleArray<V> {
    /**
     * Creates a double array.
     */
    fn _new() -> Self {
        todo!()
    }
}

impl<V> Debug for DoubleArray<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("DoubleArray")
            .field("storage", &"Box<dyn Storage<V>")
            .finish()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn new() {
        // TODO: Implement it.
        // let double_array = DoubleArray::<i32>::new();

        // assert_eq!(base_check_array_of(double_array.storage()), EXPECTED_EMPTY_BASE_CHECK_ARRAY_EMPTY);
    }
}
