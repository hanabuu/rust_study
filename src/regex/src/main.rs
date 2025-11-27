use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$")?; // YYYY-MM-DD
    let s = "2025-11-27";
    println!("is date? {}", re.is_match(s)); // true
    Ok(())
}