use std::io::{stdin, stdout, Write};
use scraper::{Selector, Html};

const TITLE: &str = " - SCP Metatitle Checker - ";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}\n{}\n{}", "=".repeat(TITLE.len()), TITLE, "=".repeat(TITLE.len()));
    print!("Please input the item number of the SCP yoou wont to check : ");
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let mut item_number: usize = input.trim().parse()?;
    if item_number < 100 { item_number -= 1 };

    let body = reqwest::blocking::get("http://scp-jp.wikidot.com/scp-series/")?.text()?;
    let document = Html::parse_document(&body);

    let content_panel = document.select(&Selector::parse("div.content-panel").unwrap()).nth(0).unwrap();
    let ul = content_panel.select(&Selector::parse("ul").unwrap()).nth(item_number / 100 + 1).unwrap();
    let li = ul.select(&Selector::parse("li").unwrap()).nth(item_number % 100).unwrap();

    println!("{}", li.text().collect::<String>());
    Ok(())
}
