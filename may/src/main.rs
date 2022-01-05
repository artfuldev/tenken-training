use may_minihttp::{HttpServer, HttpService, Request, Response};
use std::io;

/// `HelloWorld` is the *service* that we're going to be implementing to service
/// the HTTP requests we receive.
///
#[derive(Clone)]
struct HelloWorld;

impl HttpService for HelloWorld {
    fn call(&mut self, _req: Request, rsp: &mut Response) -> io::Result<()> {
        rsp.body("Tenken Web Server");
        Ok(())
    }
}

fn main() {
    may::config()
        .set_pool_capacity(10000)
        .set_stack_size(0x1000);
    let server = HttpServer(HelloWorld).start("127.0.0.1:8080").unwrap();
    server.wait();
}
