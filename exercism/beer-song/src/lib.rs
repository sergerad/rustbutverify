pub fn verse(n: u32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_owned(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_owned(),
        _ => format!("{} {} of beer on the wall, {} {} of beer.\nTake one down and pass it around, {} {} of beer on the wall.\n", n, bottles(n), n, bottles(n), n-1, bottles(n-1)),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let verses: Vec<String> = (end..=start).map(verse).rev().collect();
    verses.join("\n")
}

fn bottles(n: u32) -> String {
    match n {
        1 => "bottle".to_owned(),
        _ => "bottles".to_owned(),
    }
}
