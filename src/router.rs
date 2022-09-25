use crate::handler::{Handler, PageNotFoundHandler, StaticPageHandler};
use crate::record::Record;
use crate::request::{HttpRequest, Method, Resource};
use crate::response::HttpResponse;
use log::info;
use std::net::TcpStream;

pub struct Router;

impl Router {
    /// route various request
    pub fn route(req: HttpRequest, stream: &mut TcpStream) -> () {
        let ip = stream.local_addr().unwrap().ip().to_string();
        match &req.method {
            Method::Get|Method::Post => match &req.resource {
                Resource::Path(_) => {
                    let resp: HttpResponse = StaticPageHandler::handle(&req);
                    let _ = resp.send_response(stream);
                    // 将相关信息写入日志文件
                    info!("{}", Record::from(&req, &resp, ip));
                }
            },
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
                info!("{}", Record::from(&req, &resp, ip));
            }
        }
    }
}
