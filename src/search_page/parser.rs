use crate::helpers::{build_client_with_auto_ua, fetch_document, parse_selector};
#[cfg(feature = "atwiki")]
#[cfg(feature = "async")]
use crate::LyricsFetchError;

const SELECTOR_STR: &str = "a[title][href^='/hmiku/?cmd=word']";

pub(crate) async fn search_song_url_in_atwiki(query: &str) -> Result<String, LyricsFetchError> {
	use crate::search_page::url::build_search_url;
	let selector = parse_selector(SELECTOR_STR)?;
	// user agent is required to access atwiki
	let client = build_client_with_auto_ua();

	let url = build_search_url(query);

	let document = fetch_document(client, &url).await?;

	let result = document
		.select(&selector)
		.find(|element| element.text().collect::<String>().contains(query))
		.and_then(|element| element.value().attr("href"))
		.map(|href| format!("https://w.atwiki.jp{}", href));

	if let Some(result) = result {
		return Ok(result);
	};
	Err(LyricsFetchError::NotFound)
}

#[tokio::test]
async fn test_search_atwiki() {
	let search_word = "孤独毒毒";
	let result = search_song_url_in_atwiki(search_word).await;
	dbg!(&result);
	assert!(result.is_ok());
}
