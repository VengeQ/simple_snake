use std::fs::{File, DirBuilder};
use std::io::{Read, Write};
use regex::Regex;


fn read_from_file_or_create(file_name: &str) -> String {
    let mut content = String::new();
    let file = File::open(file_name);
    if file.is_ok() {
        file.unwrap().read_to_string(&mut content).unwrap();
        if &content == "" { content = "0".to_owned() }
    } else {
        let re = Regex::new(r"^(?P<f>(?P<d>(\w+/)*)[\w|\.]+)$").unwrap();
        match re.captures(file_name) {
            Some(captures) => {
                info!("Create new file for scores: {}", file_name);
                DirBuilder::new().recursive(true).create(&captures["d"]).unwrap();
                File::create(file_name).unwrap();
            }
            None => {
                warn!("Can't open file and parse name. Create file with default name: scores.txt");
                File::create("scores.txt").unwrap();
                content = "0".to_owned();
            }
        };
    }
    content
}

pub fn current_high_scores(file_name: &str) -> String {
    let scores = read_from_file_or_create(file_name);
    info!("current record: {}", &scores);
    let re = Regex::new(r"^(?P<v>\d+)$").unwrap();
    //scores.matches("")
    if re.is_match(&scores) { scores.to_owned() } else { "0".to_owned() }
}

pub fn update_high_scores(file_name: &str, high_scores: &str) {
    let mut file = File::create(file_name).unwrap();
    file.write_all(high_scores.as_bytes()).unwrap();
}


#[cfg(test)]
mod test {
    use crate::snake_game::scores::read_from_file_or_create;

    #[test]
    fn read_from_file_or_create_test_smoke() {
        let a = read_from_file_or_create("scores/scores.txt");
    }

    #[test]
    fn current_high_scores_test() {}
}