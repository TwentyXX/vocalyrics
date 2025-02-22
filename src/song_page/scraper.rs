use crate::LyricsFetchError;
const LYRICS_SELECTOR: &str =
	"h3#id_0a172479 ~ div:has(~ h3):not(:has(a, :not(br, ruby, rt, rb, rp, span)))";

#[cfg(feature = "atwiki")]
#[cfg(feature = "async")]
pub(crate) async fn parse_song_atwiki(url: &str) -> Result<Vec<String>, LyricsFetchError> {
	let client = reqwest::Client::builder()
		.user_agent(
			"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) \
			 Chrome/58.0.3029.110 Safari/537.3",
		)
		.build();
	let Ok(client) = client else {
		return Err(LyricsFetchError::NetworkError);
	};
	let res = client.get(url).send().await;
	let Ok(res) = res else {
		return Err(LyricsFetchError::NetworkError);
	};
	let body = res.text().await;
	let Ok(body) = body else {
		return Err(LyricsFetchError::NetworkError);
	};
	let document = scraper::Html::parse_document(&body);

	let selector = &scraper::Selector::parse(LYRICS_SELECTOR);

	let Ok(selector) = selector else {
		return Err(LyricsFetchError::SelectorError);
	};

	let lyrics = document
		.select(selector)
		.map(|element| element.text().collect())
		.collect();

	Ok(lyrics)
}

#[tokio::test]
async fn test_parse_song_atwiki() {
	let url = "https://w.atwiki.jp/hmiku/pages/38863.html";
	let result = parse_song_atwiki(url).await;
	dbg!(&result);
	assert!(result.is_ok());
}
