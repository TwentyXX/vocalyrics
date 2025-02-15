use reqwest;
use scraper;
#[cfg(feature = "atwiki")]
#[cfg(feature = "async")]
pub(crate) async fn search_song_url_in_atwiki(query: &str) -> anyhow::Result<String> {
	use anyhow::bail;

	use crate::search_page::url::build_search_url;
	let selector_str = "a[title][href^='/hmiku/?cmd=word']".to_owned();
	let selector = scraper::Selector::parse(&selector_str).unwrap();
	let url = build_search_url(query);
	// user agent is required to access atwiki
	let client = reqwest::Client::builder()
		.user_agent(
			"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) \
			 Chrome/58.0.3029.110 Safari/537.3",
		)
		.build()?;
	let res = client.get(&url).send().await?;
	let body = res.text().await?;
	let document = scraper::Html::parse_document(&body);
	let mut result = None;
	for element in document.select(&selector) {
		let text = element.text().collect::<String>();
		if text.contains(query) {
			let href = element.value().attr("href").unwrap();
			result = Some(format!("https://w.atwiki.jp{}", href));
			break;
		}
	}
	if let Some(result) = result {
		return Ok(result);
	};
	bail!("No result found");
}

#[tokio::test]
async fn test_search_atwiki() {
	let search_word = "孤独毒毒";
	let result = search_song_url_in_atwiki(search_word).await;
	dbg!(&result);
	assert!(result.is_ok());
}
