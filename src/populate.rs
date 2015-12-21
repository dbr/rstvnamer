extern crate tvdb;
use super::Date;
use super::parsing::{SeasonBased, DateBased, ParsedFile};

use tvdb::TvdbError;

/// Episode with complete set of information, usually expanded from a
/// ParsedFile instance
#[derive(Debug)]
pub struct PopulatedFile {
    pub series: String,
    pub season: i32,
    pub episode: i32,
    pub episodename: String,
    pub airdate: Date,
}


fn _populate_seasonbased(file: SeasonBased) -> Result<PopulatedFile, TvdbError>{
    let api = tvdb::Tvdb::new("0629B785CE550C8D");
    let sr = try!(api.search(file.series, "en".to_owned()));
    if sr.len() == 0 {
        return Err(TvdbError::SeriesNotFound);
    }

    let sr = sr[0].clone();

    let pf = PopulatedFile {
        series: sr.seriesname,
        season: file.season,
        episode: file.episode,
        episodename: "FIXME".to_string(),
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
