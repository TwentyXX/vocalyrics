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

	let url_string = build_search_url(query);

	let document = fetch_document(client, &url_string).await?;

	let Some(result) = document
		.select(&selector)
		.find(|element| element.text().collect::<String>().contains(query))
		.and_then(|element| element.value().attr("href"))
		.map(|href| {
			let base_url = "https://w.atwiki.jp";
			let full_url = format!("{}{}", base_url, href);
			let Ok(parsed_url) = reqwest::Url::parse(&full_url) else {
				return Err(LyricsFetchError::ParseError);
			};
			let Some(pageid) = parsed_url.query_pairs().find(|(key, _)| key == "pageid") else {
				return Err(LyricsFetchError::ParseError);
			};

			let pageid = pageid.1.to_string();
			let new_url = format!("https://w.atwiki.jp/hmiku/pages/{}.html", pageid);
			Ok(new_url)
		})
	else {
		return Err(LyricsFetchError::NotFound);
	};
	return result;
}

#[tokio::test]
async fn test_search_atwiki() {
	let search_word = "孤独毒毒";
	let result = search_song_url_in_atwiki(search_word).await;
	dbg!(&result);
	assert!(result.is_ok());
}
