/// Represents a file as a String
/// Returns a String that represents a input file (from argv)
///
/// # Args:
/// * `f_name` - A String that holds the file_path
///
/// # Examples
///
/// ```
///     let args: Vec<String> = args().collect();
///     let file: String = lexer::get_lexer_data_buffer(&args[1]);
/// ```
///
/// This is just purely used for taking in a file
pub fn get_lexer_data_buffer(f_name: &String) -> String {
    // I am pretty sure that this function is useless pretty much everywhere else
    // that is why I have decided to call it this.
    let contents = std::fs::read_to_string(&f_name).expect("Could not read file correctly");
    // return the collected chars. heathen. no returns statement
    contents.chars().collect()
}