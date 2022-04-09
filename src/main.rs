extern crate clap;

use actix_files;
use actix_web;

struct Config {
    host: String,
    port: String,
    dir: String,
    path: String,
}

fn main() {
    // コマンドライン引数を取得する
    let args_matches = get_args_matches();

    // 引数から設定を読み込む
    let config = get_config_from_args(args_matches);

    // webサーバーを起動する
    let result = actual_main(config);

    // 終了する
    std::process::exit(result);
}

fn get_args_matches() -> clap::ArgMatches<'static> {
    clap::App::new("ssfd-rs")
        .version("0.0.1")
        .author("Didy")
        .about("Rust製の静的ファイル用HTTPサーバー")
        .arg(
            clap::Arg::with_name("host")
                .help("ホストアドレス")
                .short("h")
                .long("host")
                .takes_value(true)
                .default_value("0.0.0.0"),
        )
        .arg(
            clap::Arg::with_name("port")
                .help("ポート番号")
                .short("p")
                .long("port")
                .takes_value(true)
                .default_value("8080"),
        )
        .arg(
            clap::Arg::with_name("directory")
                .help("配信元のディレクトリ")
                .short("d")
                .long("dir")
                .takes_value(true)
                .default_value("./"),
        )
        .arg(
            clap::Arg::with_name("path")
                .help("パス")
                .short("b")
                .long("path")
                .takes_value(true)
                .default_value("/"),
        )
        .get_matches()
}

fn get_config_from_args(args_matches: clap::ArgMatches<'static>) -> Config {
    Config {
        host: args_matches.value_of("host").unwrap().to_string(),
        port: args_matches.value_of("port").unwrap().to_string(),
        dir: args_matches.value_of("directory").unwrap().to_string(),
        path: args_matches.value_of("path").unwrap().to_string(),
    }
}

fn actual_main(config: Config) -> i32 {
    if let Err(err) = run(config) {
        eprintln!("{}", err);
        1
    } else {
        0
    }
}

#[actix_rt::main]
async fn run(config: Config) -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info,actix_web=info");
    env_logger::init();

    let addr = format!("{}:{}", config.host, config.port);
    let path = config.path;
    let dir = config.dir;

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(
                actix_files::Files::new(&path.to_string(), dir.to_string()).show_files_listing(),
            )
    })
    .bind(addr)?
    .run()
    .await
}
