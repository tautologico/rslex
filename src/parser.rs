use regex::Regex;
use std::collections::HashMap;

fn build_rules(path: &str) -> HashMap<String, String> {
	let mut map_rules = HashMap::new();

	//ASCII character classes
	map_rules.insert("alnum".to_string(), "[0-9A-Za-z]".to_string());
	map_rules.insert("alpha".to_string(), "[A-Za-z]".to_string());
	map_rules.insert("ascii".to_string(), "[\\x00-\\x7F]".to_string());
	map_rules.insert("blank".to_string(), "[\\t ]".to_string());
	map_rules.insert("cntrl".to_string(), "[\\x00-\\x1F\\x7F]".to_string());
	map_rules.insert("digit".to_string(), "[0-9]".to_string());
	map_rules.insert("graph".to_string(), "[!-~]".to_string());
	map_rules.insert("lower".to_string(), "[a-z]".to_string());
	map_rules.insert("print".to_string(), "[ -~]".to_string());
	map_rules.insert("punct".to_string(), "[!-/:-@[-`{-~]".to_string());
	map_rules.insert("space".to_string(), "[\\t\\n\\v\\f\\r ]".to_string());
	map_rules.insert("upper".to_string(), "[A-Z]".to_string());
	map_rules.insert("word".to_string(), "[0-9A-Za-z_]".to_string());	
	map_rules.insert("xdigit".to_string(), "[0-9A-Fa-f]".to_string());	

	map_rules
}

// s  : String 
// *s : str (via Deref<Target=str>)
// &*s: &st
fn expand_definitions(s: &str, map: HashMap<String, String>) -> String {
	let re = Regex::new(r"(\{[:alpha:]{1,}\})").unwrap();
	let mut text = String::from(s);

	for cap in re.captures_iter(s) {
		let tmp = cap.at(1).unwrap_or(""); 
		let key = &tmp[1..tmp.len()-1];

		text = text.replace(tmp, &map[key]);
	}
	text
}

#[test]
fn test_expand_def(){
	let rules: HashMap<String, String> = build_rules("path");

	let formatted1 = expand_definitions("{digit}+ and {lower}.*", rules.clone());
	let formatted2 = expand_definitions("{word}.* or {alnum}", rules.clone());
	let formatted3 = expand_definitions("{ascii} and {blank}.*", rules.clone());
	
	assert_eq!(formatted1, "[0-9]+ and [a-z].*");
	assert_eq!(formatted2, "[0-9A-Za-z_].* or [0-9A-Za-z]");
    assert_eq!(formatted3, "[\\x00-\\x7F] and [\\t ].*");
}