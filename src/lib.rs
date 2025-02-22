mod search_page;
mod song_page;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LyricsFetchError {
	NotFound,
	NetworkError,
	ParseError,
	SelectorError,
}

pub async fn fetch_vocaloid_lyrics(title: &str) -> Result<Vec<String>, LyricsFetchError> {
	let url = search_page::parser::search_song_url_in_atwiki(title).await?;
	let lyrics = song_page::scraper::parse_song_atwiki(&url).await?;
	Ok(lyrics)
}

#[tokio::test]
async fn test_fetch_vocaloid_lyrics() {
	let title = "ageha";
	let lyrics = fetch_vocaloid_lyrics(title).await;
	dbg!(&lyrics);
	assert!(lyrics.is_ok());
}
