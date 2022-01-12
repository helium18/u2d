use serde::Serialize;
use std::{env, error::Error};

use scraper::{Html, Selector};

#[derive(Serialize)]
struct Info {
    version: String,
    hash: String,
    link: String,
}

fn main() {
    let link = env::args()
        .into_iter()
        .filter(|arg| arg.starts_with("https://"))
        .next()
        .unwrap();

    let html = get_html(link.as_str()).unwrap();
    let site_links = parse_site_link(html);
    let mut json_unparsed: Vec<Info> = vec![];

    site_links.into_iter().for_each(|link| {
        let html = get_html(&link).unwrap();
        let version = parse_version(html.clone());
        let hash = parse_hash(html.clone());
        let link = parse_download_link(html.clone());
        let info = Info {
            version: version,
            hash: hash,
            link: link,
        };
        json_unparsed.push(info);
    });

    let json = serde_json::to_string_pretty(&json_unparsed).unwrap();

    println!("{}", json);
}

fn parse_hash(html: Html) -> String {
    let full_class_selector = Selector::parse("div .full").unwrap();
    let mut hash = String::new();

    let div_selector = Selector::parse("div").unwrap();

    html.select(&full_class_selector).for_each(|full_class| {
        let html = Html::parse_document(full_class.inner_html().as_str());
        html.select(&div_selector).for_each(|div| {
            if div.inner_html().len() == 64 {
                hash = div.inner_html();
            }
        })
    });

    hash
}

fn parse_version(html: Html) -> String {
    // body > div > div.detail > div.info > div.version
    let version_selector = Selector::parse("div.version").unwrap();
    let mut versions = html
        .select(&version_selector)
        .into_iter()
        .map(|version_selector| version_selector.inner_html());

    versions.next().unwrap()
}

fn parse_download_link(html: Html) -> String {
    // #detail-download-button
    let detail_download_button_selector = Selector::parse("#detail-download-button").unwrap();

    let mut download_links = vec![];

    html.select(&detail_download_button_selector)
        .into_iter()
        .for_each(|detail_download_button| {
            let link = detail_download_button.html();
            link.split(" ")
                .filter(|x| x.starts_with("href"))
                .for_each(|link| {
                    let index = link.len() - 1;
                    let link = &link[5..index];
                    let link = link
                        .replace(">", "")
                        .replace("<h3>Download</h3>", "")
                        .replace("<", "")
                        .replace("h3Download/h3", "")
                        .replace("\"", "");

                    let link = link.trim().to_string();

                    download_links.push(link);
                })
        });

    download_links[0].clone()
}

fn parse_site_link(html: Html) -> Vec<String> {
    //#versions-items-list > div:nth-child(1)
    let versions_items_list_selector = Selector::parse("#versions-items-list").unwrap();
    let div_selector = Selector::parse("div").unwrap();

    let mut links = vec![];

    html.select(&versions_items_list_selector)
        .into_iter()
        .for_each(|versions_items_list| {
            let html = Html::parse_document(&versions_items_list.inner_html());
            html.select(&div_selector).into_iter().for_each(|div| {
                let html = div.html();
                html.split(" ")
                    .filter(|x| x.starts_with("data-url"))
                    .for_each(|x| {
                        let link = x.to_string();
                        let index = link.find(">").unwrap() - 1;
                        let link = &link[10..index];
                        links.push(link.to_string())
                    });
            });
        });

    links
}

fn get_html(url: &str) -> Result<Html, Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?.text()?;
    let html = Html::parse_document(&response);
    Ok(html)
}
