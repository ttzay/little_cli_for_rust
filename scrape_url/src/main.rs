
use std::{error::Error, io::Write};
use clap::{Parser};
use reqwest::Url;
#[derive(Parser)]
#[command(version = "0.1.0", author = "zhou_yuz", about = "convert html 2 md", long_about = None)]
struct Cli {
    url_string:String,
    #[arg(short, long )]
    location:Option<String>,
    #[arg(short, long)]
    output:Option<String>
}

fn parse_url(s:&str) -> Result<Url,Box<dyn Error>> {
    let ret = Url::parse(s)?;
    Ok(ret)
}

fn parse_path(s:&str) -> Result<std::path::PathBuf,Box<dyn Error>> {
    let ret = std::path::PathBuf::from(s);
    Ok(ret)
}



fn main() {
    let args = Cli::parse();
    let url = parse_url(&args.url_string).expect("args.url_string is not a valid url");
    let filename = match args.output {
        Some(s) => s,
        None => {"output.md".to_string()}  
    }; 
    let output  = match args.location {
        Some(s) => parse_path(&s).expect("args.location is not a valid path").join(filename),
        None => {std::env::current_dir().unwrap().join(filename)}
    };

    let body = reqwest::blocking::get(url).expect("request failed").text().expect("response body is not valid utf-8");
    let md = html2md::parse_html(&body);
    let mut file2write = std::fs::File::create(&output).expect("create file failed");
    file2write.write(md.as_bytes()).expect("write file failed");
    println!("write file to {}", output.display());
}