fn main() {

    let response = reqwest::blocking::get(
        "https://www.espn.com/nba/scoreboard",
    )
    .unwrap()
    .text()
    .unwrap();
    let document = scraper::Html::parse_document(&response);

    let title_selector = scraper::Selector::parse("h3.lister-item-header>a").unwrap();
    let titles = document.select(&title_selector).map(|x| x.inner_html());

    titles
    .zip(1..101)
    .for_each(|(item, number)| println!("{}. {}", number, item));
}