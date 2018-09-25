# callgrind

[callgrind] is a Rust crate intended to complement the [valgrind_request] crate. It is intended to be used during testing to gather instrumentation and statistics to allow examination of performance bottlenecks.

To use it:-

```
extern crate callgrind;

use ::callgrind::CallgrindClientRequest;


#[test]
fn some_test()
{
	CallgrindClientRequest::start();
	
	// Execute some code to gather statistics on.
	...
	
	CallgrindClientRequest::stop(None);
}
```


## Licensing

The license for this project is MIT.

[callgrind]: https://github.com/lemonrock/callgrind "callgrind GitHub page"
[valgrind_request]: https://github.com/edef1c/libvalgrind_request "valgrind_request GitHub page"