/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache License 2.0
 **********************************************************************************************************************/

//! # Default panic handler implementation
//!
//! This module provides panic handler and personality function for a baremetal kernel that does not provide his own.
//! It will be compiled into the final binary if the feature "with_panic" is active

use core::panic::PanicInfo;
use log::*;

#[panic_handler]
#[allow(clippy::empty_loop)]
fn panic(info: &PanicInfo) -> ! {
    // Panicing is undefined behaviour so we are unable to recover from one into a valid state.
    // Halt the panicing core and safely do nothing!
    if let Some(location) = info.location() {
        // those macro's from the log crate would require that somewhere in the crate tree
        // of the final binary a Logger is implemented and configured to be used, otherwise
        // this will output nothing
        error!(
            "paniced at {:?}, {}:{}",
            location.file(),
            location.line(),
            location.column()
        );
    } else {
        error!("paniced at an unknown position");
    }

    loop {}
}

#[lang = "eh_personality"]
fn eh_personality() {
    // for the time beeing - nothing to be done as the usecase is a bit unclear
}
