// SPDX-License-Identifier: GPL-2.0

//! Rust LKM Template
//!
//! A modern template for writing Linux Kernel Modules in Rust.

#![no_main]
#![deny(missing_docs)]

mod module;

use kernel::prelude::*;

module! {
    type: RustLkmTemplate,
    name: "rust_lkm_template",
    author: "Pablo Alessando Santos Hugen",
    description: "Rust Kernel Module Template",
    license: "GPL",
}

struct RustLkmTemplate(&'static str);

impl kernel::Module for RustLkmTemplate {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        let message: &'static str = "Hello World!";
        module::test();
        pr_info!("Rust LKM Template (init)\n");
        pr_info!("Am I built-in? {}\n", !cfg!(MODULE));

        Ok(RustLkmTemplate(message))
    }
}

impl Drop for RustLkmTemplate {
    fn drop(&mut self) {
        pr_info!("My message is: {}\n", self.0);
        pr_info!("Rust LKM Template (exit)\n");
    }
}
