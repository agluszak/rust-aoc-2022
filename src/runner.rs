use std::env;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use clap::Parser;

use crate::Lines;
use reqwest::cookie::Jar;
use reqwest::{Client, ClientBuilder, Url};
use scraper::{Html, Selector};
use tokio::fs::{create_dir_all, File as TokioFile};
use tokio::io::AsyncWriteExt;

#[derive(Clone, Copy, Debug)]
pub enum Part {
    One,
    Two,
}

impl Part {
    pub fn part_number(&self) -> u8 {
        match self {
            Part::One => 1,
            Part::Two => 2,
        }
    }
}

impl FromStr for Part {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "one" | "1" => Ok(Part::One),
            "two" | "2" => Ok(Part::Two),
            _ => Err("The part must be either 'one', 1 or 'two', 2"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Day(u8);

impl Day {
    pub fn day_number(&self) -> u8 {
        self.0
    }
}

fn parse_day(s: &str) -> Result<Day, &'static str> {
    match u8::from_str(s) {
        Ok(day) if (1..=25).contains(&day) => Ok(Day(day)),
        _ => Err("The day must be a number from 1 to 25"),
    }
}

#[derive(Parser, Clone, Copy, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    #[arg(short, long, value_parser(parse_day))]
    pub day: Day,
    #[arg(short, long)]
    pub part: Part,
    #[arg(short, long)]
    pub force_download: bool,
    #[arg(short, long)]
    pub send: bool,
}

fn line_reader(path: &Path) -> Result<Lines> {
    let file = File::open(path)?;
    Ok(Lines::new(Box::new(BufReader::new(file).lines().flatten())))
}

pub struct Runner {
    client: Client,
}

impl Runner {
    pub fn new() -> Result<Self> {
        let session_key = env::var("SESSION_COOKIE")
            .map_err(|_| anyhow!("SESSION_COOKIE environment variable is not set"))?;
        let cookie = format!("session={session_key}; Domain=.adventofcode.com");
        let url = "https://adventofcode.com/".parse::<Url>()?;

        let jar = Jar::default();
        jar.add_cookie_str(&cookie, &url);

        let client = ClientBuilder::new()
            .cookie_provider(Arc::new(jar))
            .build()?;

        Ok(Self { client })
    }

    pub async fn get_input(&self, day: Day, force_download: bool) -> Result<Lines> {
        let path = format!("day-{}-input.txt", day.day_number());
        let path = Path::new(".").join("inputs").join(&path);
        let prefix = path.parent().unwrap();
        create_dir_all(prefix).await?;
        let path = path.as_path();

        if !force_download && path.exists() {
            println!("Using cached input for day {}", day.day_number());
            return line_reader(path);
        }

        println!("Downloading input for day {}", day.day_number());
        let url = format!(
            "https://adventofcode.com/2022/day/{}/input",
            day.day_number()
        )
        .parse::<Url>()?;
        let resp = self.client.get(url).send().await?;
        resp.error_for_status_ref()?;
        println!("Input downloaded");

        let mut file = TokioFile::create(path).await?;
        file.write_all(resp.text().await.unwrap().as_bytes())
            .await?;
        file.sync_all().await?;
        line_reader(path)
    }

    pub async fn send_result(&self, day: Day, part: Part, result: String) -> Result<()> {
        let url = format!(
            "https://adventofcode.com/2022/day/{}/answer",
            day.day_number()
        )
        .parse::<Url>()?;
        let resp = self
            .client
            .post(url)
            .form(&[
                ("level", part.part_number().to_string()),
                ("answer", result),
            ])
            .send()
            .await?;
        resp.error_for_status_ref()?;

        let document = Html::parse_document(&resp.text().await?);
        let selector = Selector::parse("body > main > article > p:nth-child(1)").unwrap();
        let response_text = document
            .select(&selector)
            .next()
            .ok_or_else(|| anyhow!("No content!"))?
            .text()
            .collect::<Vec<_>>()
            .join("\n");
        println!("{response_text}");
        Ok(())
    }
}
