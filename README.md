## Description
Lyrics scraper, primarily for Vocaloid content.
By default, atwiki is used as the source.
We plan to make other sources selectable, but that is not currently possible.

## Usage:

```rust
#[tokio::test]
async fn test_fetch_vocaloid_lyrics() {
	let title = "ageha";
	let lyrics = fetch_vocaloid_lyrics(title).await;
	dbg!(&lyrics);
	assert!(lyrics.is_ok());
}
```