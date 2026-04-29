// SPDX-License-Identifier: GPL-2.0

//! Module utilities for the Rust LKM Template.

use kernel::prelude::*;

/// Test function to verify the module compiles and links correctly.
pub fn test() {
    pr_info!("Rust LKM Template (test)\n");
}
