use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Ok, Result};
use clap::Parser;
use colored::*;
use mime::Mime;
use reqwest::{header, Client, Response, Url};

#[derive(Debug, Parser)]
#[command(name = "httpie", version, author, about, long_about = None)]
pub struct Opts {
    //使用subcommand标记该成员为子命令
    #[command(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

#[derive(Parser, Debug)]
struct Get {
    #[arg(value_parser = parse_url)]
    url: String,
}

#[derive(Parser, Debug)]
struct Post {
    #[arg(value_parser = parse_url)]
    url: String,
    #[arg(value_parser = parse_kv_pair)]
    body: Vec<KvPair>,
}

#[derive(Debug, Clone, PartialEq)]
struct KvPair {
    k: String,
    v: String,
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;

    Ok(s.into())
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let rsp = client.get(&args.url).send().await?;
    //println!("{:?}", rsp.text().await?);
    Ok(print_rsp(rsp).await?)
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let rsp = client.get(&args.url).json(&body).send().await?;
    //println!("{:?}", rsp.text().await?);
    Ok(print_rsp(rsp).await?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);

    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERED_BY", "RUST".parse()?);
    headers.insert(header::USER_AGENT, "RUST HTTPIE".parse()?);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let result = match opts.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };

    Ok(result)
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn print_status(rsp: &Response) {
    let status = format!("{:?} {}", rsp.version(), rsp.status().to_string().blue());
    println!("{}\n", status);
}

fn print_headers(rsp: &Response) {
    for (name, val) in rsp.headers() {
        println!("{} {:?}", name.to_string().green(), val);
    }
    println!("\n");
}

fn print_body(m: Option<Mime>, body: &String) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan())
        }
        _ => println!("{}", body),
    }
}

async fn print_rsp(rsp: Response) -> Result<()> {
    print_status(&rsp);
    print_headers(&rsp);
    let mime = get_content_type(&rsp);
    let body = rsp.text().await?;
    print_body(mime, &body);
    Ok(())
}

fn get_content_type(rsp: &Response) -> Option<Mime> {
    rsp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_works() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("http://abc.xyz").is_ok());
        assert!(parse_url("https://httpbin.org/post").is_ok());
    }

    #[test]
    fn parse_kv_pair_works() {
        assert!(parse_kv_pair("a").is_err());
        assert_eq!(
            parse_kv_pair("a=1").unwrap(),
            KvPair {
                k: "a".into(),
                v: "1".into()
            }
        );
        assert_eq!(
            parse_kv_pair("b=").unwrap(),
            KvPair {
                k: "b".into(),
                v: "".into()
            }
        );
    }
}
