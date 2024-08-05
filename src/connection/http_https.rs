pub async fn http_https_get_file(url: String) -> String {
    reqwest::get(url).await.unwrap().text().await.unwrap()
}
