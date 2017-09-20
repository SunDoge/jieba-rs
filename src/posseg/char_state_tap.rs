use std::collections::HashMap as Map;

pub fn p<'a>() -> Map<char, Vec<(char, &'a str)>> {
    let mut p = Map::new();
    p.insert('\u{4e00}', vec![('B', "m"), ('S', "m")]);
    p.insert('\u{4e01}', vec![('B', "nr"), ('S', "nr")]);
    p
}
