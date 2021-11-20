use std::{collections::HashMap, fmt::Debug, str::FromStr};
use anyhow::{anyhow, Result};

use clap::Parser;
use colored::Colorize;
use mime::Mime;
use reqwest::{Client, Response, Url, header};

/// A naive httpie implementation with Rust, can you imagine how easy it is?
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Luckychacha <luckychachaa@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

/// feed get with an url and we will retrieve the response for you
#[derive(Parser, Debug)]
struct Get {
    /// HTTP 请求的 URL
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}

/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you.
#[derive(Parser, Debug)]
struct Post {
    /// HTTP 请求的 URL
    #[clap(parse(try_from_str=parse_url))]
    url: String,
    /// HTTP 请求的 body
    #[clap(parse(try_from_str=parse_kv_pair))]
    body: Vec<KvPair>,
}

/// commands like key=value can be parsed by parse_kv_pair into KvPair Struct
#[derive(Debug)]
struct KvPair {
    k: String,
    v: String,
}

/// when we implement FromStr Trait, we can use str.parse() to parse String into KvPair.
impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            // 从迭代器中取第一个结果作为 key，迭代器返回 Some(T)/None
            // 我们将其转换成 Ok(T)/Err(E)，然后用 ？ 处理错误
            k: split.next().ok_or_else(err)?.to_string(),
            // 从迭代器中取第二个结果作为 value
            v: split.next().ok_or_else(err)?.to_string(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;

    Ok(s.into())
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
    let client = Client::new();
    let result = match opts.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };

    Ok(result)
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    Ok(print_resp(resp).await?)
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    Ok(print_resp(resp).await?)
}

async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}

fn print_body(mime: Option<Mime>, body: &String) {
    match mime {
        // 对应 "application/json" 我们使用 pretty print
        Some(mime) if mime.as_ref().starts_with(mime::APPLICATION_JSON.as_ref()) => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan())
        },
        // 其他 mime 类型，直接输出
        _ => {
            println!("{}", body)
        },
    }
}

fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
    .get(header::CONTENT_TYPE)
    .map(|v| v.to_str().unwrap().parse().unwrap())
}

fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }

    println!("\n");
}

fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

// 仅在 cargo test 的时候才编译
#[cfg(test)]
mod tests {
    use super::*;

    fn parse_url_works() {
        assert!(parse_url("abc").is_err());
    }
}