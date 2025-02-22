use crate::LyricsFetchError;

pub(crate) async fn fetch_document(
	client: reqwest::Client,
	url: &str,
) -> Result<scraper::Html, LyricsFetchError> {
	let res = client.get(url).send().await;
	let Ok(res) = res else {
		return Err(LyricsFetchError::NetworkError);
	};
	let body = res.text().await;
	let Ok(body) = body else {
		return Err(LyricsFetchError::NetworkError);
	};
	let document = scraper::Html::parse_document(&body);
	Ok(document)
}

pub(crate) fn parse_selector(selector_str: &str) -> Result<scraper::Selector, LyricsFetchError> {
	let selector = scraper::Selector::parse(selector_str);
	let Ok(selector) = selector else {
		return Err(LyricsFetchError::ParseError);
	};
	Ok(selector)
}

pub(crate) fn build_client_with_auto_ua() -> reqwest::Client {
	reqwest::Client::builder()
		.user_agent(
			"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) \
			 Chrome/58.0.3029.110 Safari/537.3",
		)
		.build()
		.unwrap()
}
