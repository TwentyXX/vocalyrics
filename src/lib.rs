mod helpers;
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
	let lyrics = song_page::scraper::fetch_lyrics_from_atwiki(&url).await?;
	Ok(lyrics)
}

pub async fn fetch_vocaloid_lyrics_html(title: &str) -> Result<Vec<String>, LyricsFetchError> {
	let url = search_page::parser::search_song_url_in_atwiki(title).await?;
	let lyrics = song_page::scraper::fetch_lyrics_from_atwiki_html(&url).await?;
	Ok(lyrics)
}

#[tokio::test]
async fn test_fetch_vocaloid_lyrics() {
	let title = "ageha";
	let lyrics = fetch_vocaloid_lyrics(title).await;
	dbg!(&lyrics);
	assert!(lyrics.is_ok());
}


#[tokio::test]
async fn test_search_atwiki_with_contain_title_in_lyrics() {
	let search_word = "ルカルカ★ナイトフィーバー";
	let lyrics = fetch_vocaloid_lyrics_html(search_word).await;
	dbg!(&lyrics);
	assert!(lyrics.is_ok());
}
