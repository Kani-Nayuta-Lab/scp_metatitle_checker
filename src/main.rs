use std::io::{stdin, stdout, Write};
use scraper::{Selector, Html};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let title = " - SCP Metatitle Checker - ";
    println!("{}\n{}\n{}", "=".repeat(title.len()), title, "=".repeat(title.len()));

    print!("Please input the item number of the SCP you want to check : ");
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let mut split = input.split(':');

    let mut item_number: usize = split.next().unwrap().trim().parse()?;
    let mut url = String::from("http://scp-jp.wikidot.com/scp-series");
    match split.next() {
        Some(area) => url.push_str(&format!("-{}", area)),
        None => (),
    }

    if item_number >= 1000 {
        url = format!("{}-{}", url, (item_number / 1000 + 1).to_string());
        item_number %= 1000;
    } else {
        if item_number < 100 {
            item_number -= 1;
        }
    }

    let body = reqwest::blocking::get(&url)?.text()?;
    let document = Html::parse_document(&body);

    let content_panel = document.select(&Selector::parse("div.content-panel").unwrap()).nth(0).unwrap();
    let ul = content_panel.select(&Selector::parse("ul").unwrap()).nth(item_number / 100 + 1).unwrap();
    let li = ul.select(&Selector::parse("li").unwrap()).nth(item_number % 100).unwrap();

    println!("{}", li.text().collect::<String>());
    Ok(())
}
