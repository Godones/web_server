use crate::request::{HttpRequest, Resource};
use crate::response::HttpResponse;
use log::info;
use std::collections::HashMap;
use std::fs;
use std::path;
use subprocess::{Exec, Redirection};

pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;
    /// load static html
    fn load_file(file_name: &str) -> Option<String> {
        // let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        // let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        // let full_path = format!("{}/{}", public_path, file_name);
        let mut full_path = format!("./{}", file_name);
        if !path::Path::new(&full_path).exists() {
            full_path = format!("./{}", "404.html");
        }
        info!("path: {}", full_path);
        let contents = fs::read_to_string(full_path);
        contents.ok()
    }
    /// get content from cgi-bin
    fn from_cgi(file_name: &str) -> Option<String> {
        //执行cgi程序得到结果返回
        let out = Exec::shell(format!("python3 {}",file_name))
            .stdout(Redirection::Pipe)
            .capture()
            .unwrap()
            .stdout_str();
        let mut lines:Vec<&str>= out.split('\n').collect();
        assert_eq!(lines[0],"Content-type:text/html");
        let lines :Vec<&str>= lines.drain(2..).collect();
        let mut ans = String::new();
        lines.iter().for_each(|&s|{
            ans.push_str(s)
        });
        Some(ans)
    }
}

pub struct StaticPageHandler;
pub struct PageNotFoundHandler;

impl Handler for PageNotFoundHandler {
    fn handle(_req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}

impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();
        match route[1] {
            "" => HttpResponse::new("200", None, Self::load_file("index.html")),
            "cgi-bin" => HttpResponse::new("200", None, Self::from_cgi("cgi-bin/hello.py")),
            path => match Self::load_file(path) {
                Some(contents) => {
                    let mut map: HashMap<&str, &str> = HashMap::new();
                    if path.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    } else if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    } else {
                        map.insert("Content-Type", "text/html");
                    }
                    HttpResponse::new("200", Some(map), Some(contents))
                }
                None => HttpResponse::new("404", None, Self::load_file("404.html")),
            },
        }
    }
}
