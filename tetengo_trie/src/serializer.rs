/*!
 * A serializer/deserializer.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::error;

/**
 * A serializer.
 */
pub trait Serializer {
    /**
     * An object type.
     */
    type Object<'a>;

    /**
     * Creates an integer serializer.
     *
     * # Arguments
     * * `fe_escape` - Set true to escape 0xFE.
     */
    fn new(fe_escape: bool) -> Self;

    /**
     * Serializes an object.
     *
     * # Arguments
     * * `object` - An object.
     *
     * # Returns
     * * The serialized object.
     */
    fn serialize(&self, object: &Self::Object<'_>) -> Vec<u8>;
}

/**
 * A deserialization error.
 */
pub trait DeserializationError: error::Error {}

/**
 * A result type.
 *
 * # Type Parameters
 * * `T` - A type.
 */
pub type Result<T> = anyhow::Result<T>;

/**
 * A deserializer.
 */
pub trait Deserializer {
    /**
     * An object type.
     */
    type Object;

    /**
     * Creates an integer deserializer.
     *
     * # Arguments
     * * `fe_escape` - Set true to escape 0xFE.
     */
    fn new(fe_escape: bool) -> Self;

    /**
     * Deserializes an object.
     *
     * # Arguments
     * * `serialized` - A serialized object.
     *
     * # Returns
     * * The deserialized object.
     *
     * # Errors
     * * When it fails to deserialize the object.
     */
    fn deserialize(&self, serialized: &[u8]) -> Result<Self::Object>;
}

/**
 * A serializer selector.
 *
 * # Type Parameters
 * * `Object` - An object type.
 */
pub trait SerializerOf<Object> {
    /// The serializer type.
    type Type;
}

/**
 * A deserializer selector.
 *
 * # Type Parameters
 * * `Object` - An object type.
 */
pub trait DeserializerOf<Object> {
    /// The deserializer type.
    type Type;
}
