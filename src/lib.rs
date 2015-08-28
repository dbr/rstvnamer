extern crate regex;
use regex::Regex;

mod test;

/// Used for air-date of an episode etc
#[derive(Debug)]
pub struct Date {
    year: i32,
    month: i32,
    day: i32,
}


/// Episode information prased from a filename.
#[derive(Debug)]
pub enum ParsedFile{
    DateBased{series: String, date: Date},
    SeasonBased{series: String, season: i32, episode: i32},
}


/// Episode with complete set of information, usually expanded from a
/// ParsedFile instance
#[derive(Debug)]
pub struct PopulatedFile {
    seriesname: String,
    season: i32,
    episode: i32,
    episodename: String,
    airdate: Date,
}


/// Errors in contacting TheTVDB
#[derive(Debug)]
pub enum TvdbError {
    SeriesNotFound,
    CommunicationError{reason: String},
}


/// Takes a ParsedFile, locates additional information (episode name
/// etc) and returns a complete PopulatedFile instance
pub fn populate(f: ParsedFile) -> Result<PopulatedFile, TvdbError> {
    return match f {
        ParsedFile::DateBased{..} => 
            Err(TvdbError::CommunicationError{reason: "Because testing".to_string()}),
        ParsedFile::SeasonBased{..} =>
            return Err(TvdbError::SeriesNotFound),
    }
}


/// Parses a filename and returns a ParsedFile
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
