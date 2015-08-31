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
pub struct DateBased{
    series: String,
    date: Date,
}

#[derive(Debug)]
pub struct SeasonBased{
    series: String,
    season: i32,
    episode: i32
}

#[derive(Debug)]
pub enum ParsedFile{
    Date(DateBased),
    Season(SeasonBased),
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

extern crate hyper;
use hyper::Client;
use hyper::header::Connection;
use std::io::Read;

use hyper::Url;


fn _populate_seasonbased(file: SeasonBased) -> Result<PopulatedFile, TvdbError>{
    let client = Client::new();

    let formatted_url = format!("http://thetvdb.com/api/GetSeries.php?seriesname={}", file.series);
    let url = Url::parse(&formatted_url).ok().expect("invalid URL");
    println!("Getting {}", url);

    let mut res = client.get(url)
        // set a header
        .header(Connection::close())
        // let 'er go!
        .send();

    let mut res = match res {
        Err(e) => return Err(TvdbError::CommunicationError{reason: "Error contacting TVDB".to_owned()}), // FIXME: http://stackoverflow.com/questions/28911833/error-handling-best-practices
        Ok(r) => r
    };

    // Read the Response.
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    //println!("{}", body);

    let pf = PopulatedFile {
        seriesname: file.series,
        season: file.season,
        episode: file.episode,
        episodename: "hi".to_string(),
        airdate: Date{year: 2014, month: 12, day: 2},
    };

    return Ok(pf);
}

fn _populate_datebased(file: DateBased) -> Result<PopulatedFile, TvdbError>{
    Err(TvdbError::CommunicationError{reason: "Because testing".to_string()})
}

/// Takes a ParsedFile, locates additional information (episode name
/// etc) and returns a complete PopulatedFile instance
pub fn populate(f: ParsedFile) -> Result<PopulatedFile, TvdbError> {
    return match f {
        ParsedFile::Date(x) => return _populate_datebased(x),
        ParsedFile::Season(x) => return _populate_seasonbased(x),
    }
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
