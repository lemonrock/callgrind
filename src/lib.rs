// This file is part of callgrind. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/callgrind/master/COPYRIGHT. No part of callgrind, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of callgrind. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/callgrind/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]


//! #callgrind
//! 
//! This is a rust library.


extern crate libc;
extern crate valgrind_request;


use ::libc::c_char;
use ::std::ffi::CString;
use ::valgrind_request::*;



include!("CallgrindClientRequest.rs");
