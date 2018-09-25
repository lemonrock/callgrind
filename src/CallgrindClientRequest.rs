// This file is part of callgrind. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/callgrind/master/COPYRIGHT. No part of callgrind, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of callgrind. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/callgrind/master/COPYRIGHT.


/// A callgrind (part of valgrind) client request.
///
/// Construct a variant and execute it using now.
///
/// Start profiling using `start()` and stop using `stop()`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CallgrindClientRequest
{
	#[doc(hidden)]
	DumpStatistics,

	#[doc(hidden)]
	ZeroStatistics,
	
	/// Toggle statistics collection on or off.
	///
	/// When first called after `Self::start()`, will toggle statistics collection off.
	ToggleCollection,

	#[doc(hidden)]
	DumpStatisticsAt
	{
		profile_dump_c_string: *const c_char,
	},
	
	#[doc(hidden)]
	StartInstrumentation,
	
	#[doc(hidden)]
	StopInstrumentation,
}

impl CallgrindClientRequest
{
	/// Execute callgrind client request now.
	#[inline(always)]
	pub fn now(self) -> u64
	{
		use self::CallgrindClientRequest::*;
		
		const Base: u64 = ((b'C' as u64) << 24) + ((b'T' as u64) << 16);
		
		let (typeValue, firstArgument) = match self
		{
			DumpStatistics => (Base, 0),
			ZeroStatistics => (Base + 1, 0),
			ToggleCollection => (Base + 2, 0),
			DumpStatisticsAt { profile_dump_c_string } => (Base + 3, profile_dump_c_string as usize as u64),
			StartInstrumentation => (Base + 4, 0),
			StopInstrumentation => (Base + 5, 0),
		};
		
		unsafe
		{
			do_client_request(0, &[typeValue, firstArgument, 0, 0, 0, 0])
		}
	}
	
	/// Start.
	#[inline(always)]
	pub fn start()
	{
		use self::CallgrindClientRequest::*;
		
		ZeroStatistics.now();
		StartInstrumentation.now();
	}
	
	/// Stop with an optional name to pass when dumping statistics.
	#[inline(always)]
	pub fn stop(name: Option<&str>)
	{
		use self::CallgrindClientRequest::*;
		
		StopInstrumentation.now();
		
		if let Some(name) = name
		{
			let name = CString::new(name).unwrap();
			Self::dump_statistics(name.as_ptr());
		}
		else
		{
			DumpStatistics.now();
		}
	}
	
	#[inline(always)]
	fn dump_statistics(profile_dump_c_string: *const c_char)
	{
		CallgrindClientRequest::DumpStatisticsAt
		{
			profile_dump_c_string
		}.now();
	}
}
