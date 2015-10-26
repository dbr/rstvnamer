use super::Date;

use hyper::Client;
use hyper::header::Connection;
use std::io::Read;

use hyper::Url;

use super::parsing::*;
use super::tvdb::*;

/// Episode with complete set of information, usually expanded from a
/// ParsedFile instance
#[derive(Debug)]
pub struct PopulatedFile {
    pub seriesname: String,
    pub season: i32,
    pub episode: i32,
    pub episodename: String,
    pub airdate: Date,
}


fn _populate_seasonbased(file: SeasonBased) -> Result<PopulatedFile, TvdbError>{
    let sr = try!(super::tvdb::series_search("scrubs", super::tvdb::ConsoleInput::new()));

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
