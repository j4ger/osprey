pub fn rss_hub_url_transform(url: &str) -> String {
    let mut result = String::new();
    if url.starts_with("http://") || url.starts_with("https://") {
        result.push_str(url);
    } else {
        result.push_str("https://");
        result.push_str(url);
    }
    if !url.ends_with("/") {
        result.push('/');
    }
    result
}
