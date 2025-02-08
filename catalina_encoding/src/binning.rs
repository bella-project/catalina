// Copyright 2022-2025 the Catalina & Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use bytemuck::{Pod, Zeroable};

/// Binning header.
#[derive(Copy, Clone, Pod, Zeroable, Debug, Default)]
#[repr(C)]
pub struct BinHeader {
    pub element_count: u32,
    pub chunk_offset: u32,
}
