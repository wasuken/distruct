use crate::config::Config;
use crate::rss::fetch_feed_items;

// ここで取得系処理をまとめる
pub fn list_from_source(config: Config) -> Vec<String> {
    let url_list = config.url_list;
    let mut data = vec![];
    for url in url_list {
        let items = fetch_feed_items(url.as_str()).unwrap();
        let mut hatena_rss_list: Vec<String> = items
            .iter()
            .map(|item| item.title().unwrap().to_string())
            .collect();
        data.append(&mut hatena_rss_list);
    }
    data
}
