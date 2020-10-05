fn main() {

    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Server {
            addr
        }
    }

    fn run(self) {
        println!("server listening on: {}", self.addr);
    }
}