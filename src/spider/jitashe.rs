use crate::model::GuitarScore;
use crate::spider::base::SearchEngine;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct JitasheJsonItem {
    #[serde(rename = "@id")]
    id: String,
    description: String,
    images: Option<Vec<String>>,
    is_original: String,
    pub_date: String,
    title: String,
}

pub struct JitasheSearchEngine {
    pub base_url: String,
}

impl JitasheSearchEngine {
    pub fn new() -> Self {
        Self {
            base_url: "https://m.jitashe.org".to_string(),
        }
    }
}

impl SearchEngine for JitasheSearchEngine {
    fn search(&self, song_name: &str) -> Vec<GuitarScore> {
        let client  = reqwest::blocking::Client::builder().user_agent("Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML,like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36").build().unwrap();
        let target_url = format!("https://m.jitashe.org/search/tab/{}/", song_name);
        let resp = client.get(target_url).send().unwrap().text().unwrap();
        let html_parse = Html::parse_document(&resp);
        let list_selector = Selector::parse("ul.my_list").unwrap();
        let item_selector = Selector::parse("a").unwrap();
        let ul = html_parse.select(&list_selector).next().unwrap();
        let mut urls: Vec<String> = vec![];
        for item in ul.select(&item_selector) {
            let href = item.value().attr("href").unwrap();
            urls.push(format!("{}{}", self.base_url, href));
        }
        let mut result: Vec<GuitarScore> = vec![];
        for u in urls {
            println!("{:?}", result);
            let content = client.get(u).send().unwrap().text().unwrap();
            let html_parse = Html::parse_document(&content);
            let script_selector = Selector::parse("script[type='application/ld+json']").unwrap();
            let script = html_parse.select(&script_selector).next().unwrap();
            let item: JitasheJsonItem = serde_json::from_str(&script.inner_html()).unwrap();
            result.push(GuitarScore {
                title: item.title,
                imgs: item.images.unwrap_or(vec![]),
                source_url: item.id,
            });
        }
        result
    }
}
