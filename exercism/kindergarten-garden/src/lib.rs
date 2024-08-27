pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];

    let char_idx = students.iter().position(|&s| s == student).unwrap() * 2;

    diagram
        .lines()
        .flat_map(|row| {
            row[char_idx..=char_idx + 1].chars().map(|c| match c {
                'V' => "violets",
                'C' => "clover",
                'R' => "radishes",
                'G' => "grass",
                _ => unreachable!(),
            })
        })
        .collect::<Vec<_>>()
}
