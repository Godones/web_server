pub struct LogInfo<'a> {
    ip: &'a str,
    identity: &'a str,
    id: &'a str,
    time: &'a str,
    method: &'a str,
    resource: &'a str,
    status: &'a str,
    file_size: usize,
    content: &'a [u8],
}
impl ToString for LogInfo<'_> {
    fn to_string(&self) -> String {
        let content = String::from_utf8(self.content.to_vec()).unwrap();
        format!(
            "{} {} {} [{}] \"{} {}\" {} {} \"{}\"",
            self.ip,
            self.identity,
            self.id,
            self.time,
            self.method,
            self.resource,
            self.status,
            self.file_size,
            content
        )
    }
}
