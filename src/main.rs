
fn get_articles(url: &url){
    let response = ureq::get(url).call()?
}

fn main() {
    let url: &str = "https://https://newsapi.org/v2/everything?q=keyword&apiKey=";
    let articles = get_articles(url);
}
