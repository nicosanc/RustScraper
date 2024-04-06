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


/* fn get_quotes(){
    let url = "https://quotes.toscrape.com/";
    let resp = reqwest::blocking::get(url).unwrap().text().unwrap();

    let document = Html::parse_document(&resp);

    // Selectors are data structures that can hold some type of command
    // as a reference for when its used to parse the html dom
    let tag_selector = Selector::parse(".quote").unwrap();
    
    // Using this selector we try and find all of the quotes in the document,
    // using the class name of their css style .quote

    for quote in document.select(&tag_selector){
        // This returns an Option<ElementRef> which is a reference to an element in an element
        // in this case the individual tag elements
       if let Some(tags) = quote.select(&Selector::parse(".tags").unwrap()).next(){

           for tag in tags.select(&Selector::parse(".tag").unwrap()){
               println!("{}", tag.inner_html());
           }
       }
       println!("---")

    }


} */