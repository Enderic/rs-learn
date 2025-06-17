use trpl::{Either, Html};

// Super Basic Web Scraper!

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        // Call page_title() for each url to get futures
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        // We pass the futures into ::race() to see which finishes first
        // Runs it for us
        // We would usually use ::select() more
        // It returns an Either to show which finished first
        let (url, maybe_title) =
            match trpl::race(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    })
}

// Changed return type to include original url
async fn page_title(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}