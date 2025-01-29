mod engine;

use std::fs::{self, read_to_string};

use clap::{ArgAction::Count, Parser, Subcommand};
use engine::{
    cookies::Cookie, headers::Header, request::EngineRequest, request_engine::Engine,
    traits::Validate,
};
// use types::;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Subcommands,

    #[arg(short = 'v', action = Count)]
    verbose: u8,

    #[arg(short = 'n', long)]
    num_requests: Option<i32>,

    #[arg(short = 'j', long, default_value_t = 16)]
    num_threads: u16,

    #[arg(short, long)]
    dump_request_to_json_file: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    Raw {
        url: String,
        #[arg(long = "header")]
        headers: Vec<Header>,
        #[arg(long = "cookie")]
        cookies: Vec<Cookie>,
        #[arg(long)]
        body: Option<String>,
    },
    FromJSONFile {
        file: String,
    },
}

fn main() {
    let args = Args::parse();
    let request = match args.command {
        Subcommands::Raw {
            url,
            headers,
            cookies,
            body,
        } => EngineRequest::new(url, headers, cookies, body),
        Subcommands::FromJSONFile { file } => {
            let text = read_to_string(file).expect("Unable to open file;");
            let json = serde_json::from_str(&text);
            json.expect("Invalid JSON Data in file.")
        }
    };
    request.validate().expect("Unable to validate");
    if args.dump_request_to_json_file.is_some() {
        fs::write(
            args.dump_request_to_json_file.unwrap(),
            serde_json::to_string(&request).unwrap(),
        )
        .expect("Unable to write to file.");
    }
    let engine = Engine::new(request, args.num_requests);
    engine.run(args.num_threads);
}
