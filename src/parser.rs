use regex::Regex;
use std::collections::HashMap;

fn build_rules(path: &str) -> HashMap<&str, &str> {
	let mut map_rules = HashMap::new();

	map_rules.insert("DIGIT", "[0-9]");
	map_rules.insert("ALPHABET", "[a-z]");

	map_rules
}

// s  : String 
// *s : str (via Deref<Target=str>)
// &*s: &st
fn expand_definitions(s: &str, map: HashMap<&str, &str>) -> String {
	let re = Regex::new(r"(\{[:alpha:]{1,}\})").unwrap();
	let mut text = String::from(s);

	for cap in re.captures_iter(s) {
		let tmp = cap.at(1).unwrap_or(""); 
		let key = &tmp[1..tmp.len()-1];

		text = text.replace(tmp, map[key]);
	}
	text
}

#[test]
fn test_expand_def(){
	let rules: HashMap<&str, &str> = build_rules("");
	let formatted = expand_definitions("{DIGIT}+ not is {ALPHABET}.*", rules);

    assert_eq!(formatted, "[0-9]+ not is [a-z].*");
}