pub(crate) fn build_search_url(query: &str) -> String {
	format!(
		"https://w.atwiki.jp/hmiku/search?andor=and&keyword={}",
		urlencoding::encode(query)
	)
}

#[test]
fn test_build_search_url() {
	let query = "孤独毒毒";
	let url = build_search_url(query);
	assert_eq!(
		url,
		"https://w.atwiki.jp/hmiku/search?andor=and&keyword=%E5%AD%A4%E7%8B%AC%E6%AF%92%E6%AF%92"
	);
}
