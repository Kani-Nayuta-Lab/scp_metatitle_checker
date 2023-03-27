use scraper::{Html, Selector};
use std::io::{stdin, stdout, Write};

const TITLE: &str = " - SCP Metatitle Checker - ";
const SCP_URL: &str = "http://scp-jp.wikidot.com/scp-series";

fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!(
    "{}\n{}\n{}",
    "=".repeat(TITLE.len()),
    TITLE,
    "=".repeat(TITLE.len())
  );
  println!("Please input the item number of the SCP you want to check");
  println!("If you want to check other branches SCP, Enter ':[BRANCH_NAME]'(Ex. 544:JP) after the item number");
  print!(">>> ");
  stdout().flush()?;

  let mut input = String::new();
  stdin().read_line(&mut input)?;

  let mut split = input.split(':');
  let item_number = split.next().unwrap().trim().parse::<usize>()?;
  let area_url = if let Some(area) = split.next() {
    format!("{}-{}", SCP_URL, area.to_lowercase())
  } else {
    String::from(SCP_URL)
  };
  let (li_num, target_url) = if item_number < 1000 {
    (item_number - 1, area_url)
  } else {
    (
      item_number % 1000,
      format!("{}-{}", area_url, (item_number / 1000 + 1)),
    )
  };

  let document = Html::parse_document(&reqwest::blocking::get(&target_url)?.text()?);
  let selector = Selector::parse("div.content-panel #toc2~ul li").unwrap();

  println!(
    "{}",
    document
      .select(&selector)
      .nth(li_num)
      .expect("ERROR: Cannot find specified SCP!!")
      .text()
      .collect::<String>()
  );
  return Ok(());
}
