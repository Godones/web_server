#![feature(once_cell)]

use crate::server::Server;
use clap::Parser;
mod handler;
mod logging;
mod pool;
mod record;
mod request;
mod response;
mod router;
mod server;

/// 定义命令行参数
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// number of threads
    #[clap(short, long, value_parser, default_value_t = 4)]
    threads: usize,
}

fn main() {
    // 初始化日志
    logging::init(Some("./log/web.log"));
    // 解析命令行参数得到线程数
    let args = Args::parse();

    // 创建服务器运行
    let server = Server::new("localhost:8888");
    server.run(args.threads);
}
