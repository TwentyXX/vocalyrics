mod search_page;
mod song_page;

pub async fn get_vocaloid_lyrics(title: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
	let url = search_page::parser::search_song_url_in_atwiki(title).await?;
	let lyrics = song_page::scraper::parse_song_atwiki(&url).await?;
	Ok(lyrics)
}

#[tokio::test]
async fn test_get_vocaloid_lyrics() {
	let title = "ageha";
	let lyrics = get_vocaloid_lyrics(title).await;
	dbg!(&lyrics);
	assert!(lyrics.is_ok());
}