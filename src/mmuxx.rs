/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache License 2.0
 **********************************************************************************************************************/
//! # Dummy MMU implementation for non Aarchxx targets
//! This is mainly used as place holder to allow ``cargo fmt`` and ``cargo test`` to execute success
//! fully.
//!

pub fn initialize_mmu(_core: u32) {}
pub fn disable_mmu() {}
