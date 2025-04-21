use std::{fs, io::{self, Read, Write}, net::{TcpListener, TcpStream}};

use crate::{error, info, request::HttpRequest, warn};

pub struct HttpServer {
    port: u16,
    doc_root: String,
    ip: String
}

impl HttpServer {
    pub fn new(ip: String, port: u16, doc_root: String) -> Self {
        Self { port, doc_root, ip }
    }

    pub fn listen(&self) -> ! {
        let socket = TcpListener::bind(format!("{}:{}", self.ip, self.port)).expect("Failed to create tcp listener");
        
        loop {
            match socket.accept() {
                Ok((mut stream, _addr)) => {
                    //info!("Got a connection from {}", _addr.ip());
                    self.handle_request(&mut stream);
                }

                Err(e) => error!("{e}")
            }    
        }
    }

    fn handle_request(&self, stream: &mut TcpStream) {
        let mut buff = [0; 1024];
        let _ = stream.read(&mut buff);
        let text = String::from_utf8(buff.to_vec()).unwrap();
        let line = text.lines().nth(0).unwrap().split(" ").collect::<Vec<&str>>();

        if line.len() < 3 {
            return;
        }

        let mut request = HttpRequest::new(line[0].to_owned(), line[1].to_owned());

        if request.method == "POST".to_owned() {
            return;
        }

        if request.path.len() > 1 && request.path.contains("/") {
            request.path = request.path.strip_prefix("/").unwrap().to_owned();
        }

        info!("{} --> {}", request.method, request.path);
        let resp = self.prepare_response(&request);

        let bytes = resp.as_bytes();

        stream.write_all(bytes).expect("Could not respond");
        stream.flush().unwrap();
    }

    fn prepare_response(&self, req: &HttpRequest) -> String {
        let contents = self.load_file(&req.path);

        match contents {
            Ok(contents) => {
                format!(
                    "HTTP/1.1 200 Ok\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
                    contents.1,
                    contents.0.len(),
                    contents.0
                )
            }   

            Err(_) => {
                let body = r#"
<html>
  <head><title>404 Not Found</title></head>
  <body><h1>404 - File Not Found</h1></body>
</html>"#;

                format!(
                    "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                )
            }
        }
    }

    fn load_file(&self, request_path: &str) -> io::Result<(String, String)> {
        let paths = fs::read_dir(&self.doc_root)?;

        for path in paths {
            let dir_entry = path?;
            let req_path = if request_path == "/" { "index.html".to_owned() } else { request_path.to_owned() };
            let file_name = dir_entry.file_name();

            if req_path == file_name.to_str().unwrap() {
                let ext = req_path.split(".").last().unwrap_or("");
                let mime: &str = match ext {
                    "html" => {
                        "text/html"
                    }

                    "css" => {
                        "text/css"
                    }

                    "js" => {
                        "text/javascript"
                    }

                    _ => {
                        "text/plain"
                    }
                };  

                let mut file = fs::File::open(dir_entry.path())?;
                let mut contents = String::new();

                file.read_to_string(&mut contents)?;

                return Ok((contents, mime.to_owned()));
            }
        }

        warn!("File {} not found", request_path);
        Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))
    }
}