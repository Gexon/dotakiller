use mio::{EventLoop, io, buf};


fn main() {
			start().assert("Цикл обработки событий не может быть запущен");
}


fn start() -> MioResult<EventLoop> {
	// Create a new event loop. This can fail if the underlying OS cannot
	// provide a selector.
	let event_loop = try!(EventLoop::new());

	// Create a two-way pipe.
	let (mut reader, mut writer) = try!(io::pipe());

	// the second parameter here is a 64-bit, copyable value that will be sent
	// to the Handler when there is activity on `reader`
	try!(event_loop.register(&reader, 1u64));

	// kick things off by writing to the writer side of the pipe
	try!(writer.write(buf::wrap("hello".as_bytes())));

	event_loop.run(MyHandler)
}

struct MyHandler;

impl Handler for MyHandler {
fn readable(&mut self, _loop: &mut EventLoop, token: u64) {
	println!("The pipe is readable: {}", token);
}
}

//#[test]
//fn test_dota_mast_die() {
//
//}