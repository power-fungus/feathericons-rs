use regex::Regex;
use seed_feathericons::Icons;

fn main() {
    let sprite = Icons::SPRITE;

    let re = Regex::new(r#"<symbol id="([^"]*)" viewBox="0 0 24 24">(.*?)</symbol>"#).unwrap();
    for (i, cap) in re.captures_iter(sprite).enumerate() {
        println!("{}: {} [{}]", i, &cap[1], &cap[2]);
    }
}
