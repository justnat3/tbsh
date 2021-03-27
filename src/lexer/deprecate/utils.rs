pub fn extract_whitespace(s: &str) -> String {
    if !s.to_string().contains(char::is_whitespace) {
        return s.to_string();
    }
    let results = s.split_whitespace().collect::<String>();
    // we return a String because we do not want to return a reference
    // the ref data is owned by the function
    results
}
