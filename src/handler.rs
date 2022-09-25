use crate::request::{HttpRequest, Resource};
use crate::response::HttpResponse;
use std::collections::HashMap;
use std::{ fs};
use std::path;
use std::process::Command;

pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;
    /// load static html
    fn load_file(file_name: &str) -> Option<String> {
        let mut file_name = file_name.to_string();
        if file_name.starts_with('/') {
            file_name.remove(0);
        }
        let mut full_path = format!("{}", file_name);
        if !path::Path::new(&full_path).exists() {
            println!("{} not exist",full_path);
            full_path = format!("{}", "404.html");
        }
        let contents = fs::read(full_path);
        let ans= unsafe {String::from_utf8_unchecked(contents.unwrap())};
        Some(ans)
    }
    /// get content from cgi-bin
    fn from_cgi(file_name: &str,args:Vec<String>) -> Option<String> {
        let mut file_name = file_name.to_string();
        if file_name.starts_with('/') {
            file_name.remove(0);
        }
        args.iter().for_each(|arg|{
            file_name = format!("{} {}",file_name,arg);
        });
        println!("command: {}",file_name);
        //执行cgi程序得到结果返回
        let mut command = Command::new("sh");
        command.arg("-c")
            .arg(format!("python3 {}",file_name));
        let out = command.output().expect("failed to execute process");
        let out = String::from_utf8(out.stdout).unwrap();


        let mut lines: Vec<&str> = out.split('\n').collect();
        assert_eq!(lines[0], "Content-type:text/html");
        let lines: Vec<&str> = lines.drain(2..).collect();
        let mut ans = String::new();
        lines.iter().for_each(|&s| ans.push_str(s));
        println!("out: {}",ans);
        Some(ans)
    }
    fn args_from_body(request:&HttpRequest)->Vec<String>{
        let content_length = request.headers
            .get("Content-Length")
            .unwrap()
            .parse::<usize>()
            .unwrap_or(0);
        if content_length == 0{
            return vec![];
        }
        let body = request.msg_body.as_bytes()[0..content_length].to_vec();
        let body = String::from_utf8(body).unwrap();
        let args :Vec<&str>= body.split('&').collect();
        let args = args.iter().map(|&s|{
            s.to_string()
        }).collect();
        args
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
            "cgi-bin" => {
                // 从消息体获取参数
                let args = Self::args_from_body(req);
                HttpResponse::new("200", None, Self::from_cgi(s,args))
            }
            _ => match Self::load_file(s) {
                Some(contents) => {
                    let mut map: HashMap<&str, &str> = HashMap::new();
                    if s.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    } else if s.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    } else if s.ends_with(".jpg") {
                        println!("load img from disk");
                        map.insert("Content-type","image/jpeg");
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
