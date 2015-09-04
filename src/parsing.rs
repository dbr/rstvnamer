use super::Date;
use regex::Regex;



/// Episode information prased from a filename.
#[derive(Debug)]
pub struct DateBased{
    pub series: String,
    pub date: Date,
}

#[derive(Debug)]
pub struct SeasonBased{
    pub series: String,
    pub season: i32,
    pub episode: i32
}

#[derive(Debug)]
pub enum ParsedFile{
    Date(DateBased),
    Season(SeasonBased),
}


/// Parses a filename and returns a ParsedFile
pub fn parse(fname:&str) -> Option<ParsedFile>{
    let x = Regex::new(r"([a-zA-Z0-9]+)\.s(\d{2})e(\d{2})").unwrap();

    let caps = x.captures(fname);
    if let Some(x) = caps {
        return Some(ParsedFile::Season(SeasonBased{
            series: x.at(1).unwrap().to_string(),
            season: x.at(2).unwrap().to_string().parse::<i32>().unwrap(),
            episode: x.at(3).unwrap().to_string().parse::<i32>().unwrap(),
        }))
    }

    let x = Regex::new(r"([a-zA-Z0-9]+)\.(\d{4})\.(\d{2})\.(\d{2})").unwrap();

    let caps = x.captures(fname);
    if let Some(x) = caps {
        return Some(ParsedFile::Date(DateBased{
            series: x.at(1).unwrap().to_string(),
            date: Date{year: x.at(2).unwrap().to_string().parse::<i32>().unwrap(),
                       month: x.at(3).unwrap().to_string().parse::<i32>().unwrap(),
                       day: x.at(4).unwrap().to_string().parse::<i32>().unwrap()}
        }));
    }

    return None;
}
