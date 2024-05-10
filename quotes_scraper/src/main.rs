use reqwest;
use reqwest::Client;
use url::Url;
use scraper::{html, Html, Selector};
use std::collections::HashSet;

static base_url: &str = "https://books.toscrape.com/";
#[allow(dead_code)]
fn main() {
    let url = "https://books.toscrape.com/index.html";
    let resp = reqwest::blocking::get(url).unwrap().text().unwrap();

    // get_quotes();
    let links = find_links(&resp);
    for link in links{
        println!("page: {}", link)
    }
    let mut set = HashSet::new();
    crawl(url, &mut set)
}

fn find_links(html_content: &str) -> Vec<String>{
    let document = Html::parse_document(html_content);
    let link_select = &Selector::parse("a").unwrap();
    let mut links = Vec::new();

    for element in document.select(&link_select){
        if let Some(href) = element.value().attr("href") {
            links.push(href.to_string());
        }
    }
    links
    
}

// Make Absolute URL
fn make_absolute_links(link: &str) -> Option<String> {
    let base = Url::parse(base_url).ok()?;
    let url = base.join(link).ok()?;
    Some(url.into())
}

// CRAWL
fn crawl(url: &str, visited: &mut HashSet<String>){

    if visited.contains(url){
        return;
    }

    println!("Crawling URL: {}", url);

    visited.insert(url.to_string());

    let client = reqwest::blocking::Client::new();
    match client.get(url).send(){
        Ok(resp) => match resp.text() {
            Ok(body) => {
                let links = find_links(&body);
                for link in links{
                    if let Some(absolute_link) = make_absolute_links(&link) {
                        crawl(&absolute_link, visited);
                    }
                    else{
                        println!("Could not create an absolute_link for: {}", link);
                    }
                
                }
            }, 
            Err(_) => println!("Could not get the body of the page"),
        }, 
        Err(_) => println!("Could not make request to: {}",url),
    }
}
