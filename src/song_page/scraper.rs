use crate::LyricsFetchError;
const LYRICS_SELECTOR: &str =
	"h3#id_0a172479 ~ div:has(~ h3):not(:has(a, :not(br, ruby, rt, rb, rp, span)))";

#[cfg(feature = "atwiki")]
#[cfg(feature = "async")]
pub(crate) async fn fetch_lyrics_from_atwiki(url: &str) -> Result<Vec<String>, LyricsFetchError> {
	use crate::helpers::{build_client_with_auto_ua, fetch_document, parse_selector};

	let client = build_client_with_auto_ua();
	let document = fetch_document(client, url).await?;

	let selector = parse_selector(LYRICS_SELECTOR)?;

	let lyrics = document
		.select(&selector)
		.map(|element| element.text().collect())
		.collect();

	Ok(lyrics)
}

#[tokio::test]
async fn test_fetch_lyrics_from_atwiki() {
	let url = "https://w.atwiki.jp/hmiku/pages/38863.html";
	let result = fetch_lyrics_from_atwiki(url).await;
	dbg!(&result);
	assert!(result.is_ok());
}
