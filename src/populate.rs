extern crate tvdb;
use super::parsing::{SeasonBased, DateBased, ParsedFile};

use tvdb::Date;
use tvdb::TvdbError;

/// Episode with complete set of information, usually expanded from a
/// ParsedFile instance
#[derive(Debug)]
pub struct PopulatedFile {
    pub series: String,
    pub season: u32,
    pub episode: u32,
    pub episodename: String,
    pub airdate: Date,
}


fn _populate_seasonbased(file: SeasonBased) -> Result<PopulatedFile, TvdbError>{
    let api = tvdb::Tvdb::new("0629B785CE550C8D");
    let sr = try!(api.search(file.series, "en".to_owned()));
    if sr.len() == 0 {
        return Err(TvdbError::SeriesNotFound);
    }

    let ep = try!(api.episode(&sr[0], file.season, file.episode));
    let sn = sr[0].seriesname.clone();

    let pf = PopulatedFile {
        series: sn,
        season: file.season,
        episode: file.episode,
        episodename: ep.episodename,
        airdate: ep.firstaired.unwrap(),
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
