fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
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

    fn run(self) {
        println!("Server is started on the {}", self.addres)
    }
}

struct Request {
    path: String,
    query: String,
    method: Method,
}

enum Method {
    GET,
    POST,
    DELETE,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

/*GET /user?id=10 HTTP/1.1\r\n
HEADERS
BODY
*/