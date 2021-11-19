fn main() {
    let server = Server::new("127.0.0.1:8080");
    server.run();
}

struct Server {
    addres: String,
}

impl Server {
    fn new(address: String) -> Self {
        Server {
            addres: address
        }
    }

    fn run(self){
        println!("Server is started")
    }
}
