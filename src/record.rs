use crate::request::HttpRequest;
use crate::response::HttpResponse;
use chrono::Local;

pub struct Record;

impl Record {
    /// 构建解析得到的请求信息组装
    pub fn from(request: &HttpRequest, response: &HttpResponse, ip: String) -> String {
        let time = Local::now().format("%d/%b/%Y:%H:%M:%S").to_string();
        format!(
            "{}--[{}] {} {} {} {} {} {} {} {}",
            ip,
            time,
            request.method.to_string(),
            request.resource.to_string(),
            request.version.to_string(),
            response.status_code(),
            request.headers.get("Referer").unwrap(),
            request.resource.to_string(),
            response.body().len(),
            request
                .headers
                .get("User-Agent")
                .unwrap_or(&"unknown".to_string())
        )
    }
}
