use reqwest::blocking::Client;
use scraper::{Html, Selector};

fn load_data()
{
    let client = Client::new();

    let current_content = client
        .get("https:/espn.com/mma/fightcenter")
        .send()
        .unwrap()
        .text()
        .unwrap();

    parsing(current_content);
}

fn parsing(content: String)
{
    let document = Html::parse_document(&content);

    let fighter_selector =
        Selector::parse(r#"div[class="MMACompetitor__Detail flex flex-column justify-center"]"#)
            .unwrap();

    for element in document.select(&fighter_selector)
    {
        let name_selector = Selector::parse(r#"span[class="truncate tc db"]"#).unwrap();

        for name in element.select(&name_selector)
        {
            println!("Name: {:?}", name.inner_html());
        }

        let record_selector =
            Selector::parse(r#"div[class="flex items-center n9 nowrap justify-end clr-gray-04"]"#)
                .unwrap();
        for record in element.select(&record_selector)
        {
            println!("Record: {:?}", record.inner_html().trim());
        }

        let record_selector_right =
            Selector::parse(r#"div[class="flex items-center n9 nowrap clr-gray-04"]"#).unwrap();
        for record in element.select(&record_selector_right)
        {
            println!("Record: {:?}", record.inner_html().trim());
        }
    }
}
