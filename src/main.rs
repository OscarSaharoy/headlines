fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://en.wikipedia.org/wiki/Main_Page")?.text()?;
    let fragment = scraper::Html::parse_fragment(&resp);
    let selector = scraper::Selector::parse("#mp-itn ul").unwrap();

    let itn = fragment.select(&selector).next().unwrap();
    let mut text = itn.text().collect::<Vec<_>>().join("").replace("\n", "\n- ");
    text.insert_str(0, "- ");

    println!("headlines for {}\n", chrono::offset::Local::now().format("%a %b %e"));
    println!("{text}");
    Ok(())
}
