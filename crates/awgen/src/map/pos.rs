//! This module implements helper types for working with coordinate positions in
//! the voxel world.

use core::fmt;
use std::ops::{Add, Mul};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::map::chunk::{CHUNK_SIZE, CHUNK_SIZE_BITS, CHUNK_SIZE_MASK};

/// The position of a block in the world, represented in world-space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Serialize, Deserialize)]
pub struct WorldPos(IVec3);

impl WorldPos {
    /// Creates a new [`WorldPos`] from the given x, y, and z coordinates.
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        WorldPos(IVec3::new(x, y, z))
    }

    /// Gets the chunk position of this block in the world.
    pub fn as_chunk_pos(self) -> ChunkPos {
        ChunkPos(IVec3::new(
            self.x >> CHUNK_SIZE_BITS,
            self.y >> CHUNK_SIZE_BITS,
            self.z >> CHUNK_SIZE_BITS,
        ))
    }

    /// Gets the relative position of this block within its chunk.
    pub fn as_local_pos(self) -> LocalPos {
        LocalPos(IVec3::new(
            self.x & CHUNK_SIZE_MASK,
            self.y & CHUNK_SIZE_MASK,
            self.z & CHUNK_SIZE_MASK,
        ))
    }
}

impl fmt::Display for WorldPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add<WorldPos> for WorldPos {
    type Output = Self;

    fn add(self, rhs: WorldPos) -> Self::Output {
        WorldPos(self.0 + rhs.0)
    }
}

impl Mul<i32> for WorldPos {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        WorldPos(self.0 * rhs)
    }
}

/// The position of a chunk in the world, represented in chunk-space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, Serialize, Deserialize)]
pub struct ChunkPos(IVec3);

impl fmt::Display for ChunkPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

/// The position of a local position within a chunk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, Serialize, Deserialize)]
pub struct LocalPos(IVec3);

impl LocalPos {
    /// Gets the array index position of this block within a chunk.
    pub fn as_index(self) -> usize {
        let x = self.x as usize;
        let y = self.y as usize;
        let z = self.z as usize;
        x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE
    }
}

impl From<WorldPos> for LocalPos {
    fn from(world_pos: WorldPos) -> Self {
        world_pos.as_local_pos()
    }
}

impl<P: Into<LocalPos>> Add<P> for LocalPos {
    type Output = Self;

    fn add(self, rhs: P) -> Self::Output {
        let mut vec = self.0 + rhs.into().0;
        vec.x &= CHUNK_SIZE_MASK;
        vec.y &= CHUNK_SIZE_MASK;
        vec.z &= CHUNK_SIZE_MASK;
        LocalPos(vec)
    }
}

/// A cardinal unit direction vector in 3D space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Dir(IVec3);

impl Dir {
    /// The positive Y direction (0, 1, 0).
    pub const POS_Y: Self = Self(IVec3::new(0, 1, 0));

    /// The negative Y direction (0, -1, 0).
    pub const NEG_Y: Self = Self(IVec3::new(0, -1, 0));

    /// The positive X direction (1, 0, 0).
    pub const POS_X: Self = Self(IVec3::new(1, 0, 0));

    /// The negative X direction (-1, 0, 0).
    pub const NEG_X: Self = Self(IVec3::new(-1, 0, 0));

    /// The positive Z direction (0, 0, 1).
    pub const POS_Z: Self = Self(IVec3::new(0, 0, 1));

    /// The negative Z direction (0, 0, -1).
    pub const NEG_Z: Self = Self(IVec3::new(0, 0, -1));
}

impl From<Dir> for LocalPos {
    fn from(dir: Dir) -> Self {
        LocalPos(dir.0)
    }
}

impl From<Dir> for WorldPos {
    fn from(dir: Dir) -> Self {
        WorldPos(dir.0)
    }
}

impl Mul<i32> for Dir {
    type Output = WorldPos;

    fn mul(self, rhs: i32) -> Self::Output {
        WorldPos(self.0 * rhs)
    }
}
