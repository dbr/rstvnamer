extern crate regex;
use regex::Regex;

mod test;

#[derive(Debug)]
pub struct Date {
    year: i32,
    month: i32,
    day: i32,
}

#[derive(Debug)]
pub enum ParsedFile{
    DateBased{series: String, date: Date},
    SeasonBased{series: String, season: i32, episode: i32},
}

#[derive(Debug)]
pub struct PopulatedFile {
    seriesname: String,
    season: i32,
    episode: i32,
    episodename: String,
    airdate: Date,
}

#[derive(Debug)]
pub enum TvdbError {
    SeriesNotFound,
    CommunicationError{reason: String},
}



pub fn populate(f: ParsedFile) -> Result<PopulatedFile, TvdbError> {
    return match f {
        ParsedFile::DateBased{..} => 
            Err(TvdbError::CommunicationError{reason: "Because testing".to_string()}),
        ParsedFile::SeasonBased{..} =>
            return Err(TvdbError::SeriesNotFound),
    }
}

pub fn parse(fname:&str) -> Option<ParsedFile>{
    let x = Regex::new(r"([a-zA-Z0-9]+)\.s(\d{2})e(\d{2})").unwrap();

    let caps = x.captures(fname);
    if let Some(x) = caps {
        return Some(ParsedFile::SeasonBased{
            series: x.at(1).unwrap().to_string(),
            season: x.at(2).unwrap().to_string().parse::<i32>().unwrap(),
            episode: x.at(3).unwrap().to_string().parse::<i32>().unwrap(),
        })
    }

    let x = Regex::new(r"([a-zA-Z0-9]+)\.(\d{4})\.(\d{2})\.(\d{2})").unwrap();

    let caps = x.captures(fname);
    if let Some(x) = caps {
        return Some(ParsedFile::DateBased{
            series: x.at(1).unwrap().to_string(),
            date: Date{year: x.at(2).unwrap().to_string().parse::<i32>().unwrap(),
                       month: x.at(3).unwrap().to_string().parse::<i32>().unwrap(),
                       day: x.at(4).unwrap().to_string().parse::<i32>().unwrap()}
        });
    }

    return None;
}
