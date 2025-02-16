const TITLE_SELECTOR: &str = "h2 > a";
const LYRICS_SELECTOR: &str =
	"h3#id_0a172479 ~ div:has(~ h3):not(:has(a, :not(br,ruby, rt, rb, rp)))";

#[cfg(feature = "atwiki")]
#[cfg(feature = "async")]
pub(crate) async fn parse_song_atwiki(url: &str) -> anyhow::Result<Vec<String>> {
	let client = reqwest::Client::builder()
		.user_agent(
			"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) \
			 Chrome/58.0.3029.110 Safari/537.3",
		)
		.build()?;
	let res = client.get(url).send().await?;
	let body = res.text().await?;
	let document = scraper::Html::parse_document(&body);
	// 歌詞は複数の要素であり、それらを全て取得する
	let lyrics = document
		.select(&scraper::Selector::parse(LYRICS_SELECTOR).unwrap())
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
