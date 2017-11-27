extern crate tvdb;
use super::parsing::{DateBased, ParsedFile, SeasonBased};
use super::config::TVDB_API_KEY;
use super::errors::TvnamerResult;

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


fn _populate_seasonbased(file: &SeasonBased) -> TvnamerResult<PopulatedFile> {
    let api = tvdb::Tvdb::new(TVDB_API_KEY);
    let sr = api.search(&file.series, "en")?;
    if sr.is_empty() {
        return Err(TvdbError::SeriesNotFound.into());
    }

    let ep = api.episode(&sr[0], file.season, file.episode)?;
    let sn = sr[0].seriesname.clone();

    let pf = PopulatedFile {
        series: sn,
        season: file.season,
        episode: file.episode,
        episodename: ep.episode_name,
        airdate: ep.first_aired.unwrap(),
    };

    return Ok(pf);
}

fn _populate_datebased(file: &DateBased) -> TvnamerResult<PopulatedFile> {
    Err(
        TvdbError::CommunicationError { reason: "Because testing".into() }.into(),
    )
}

/// Takes a `ParsedFile`, locates additional information (episode name
/// etc) and returns a complete `PopulatedFile` instance
pub fn populate(f: &ParsedFile) -> TvnamerResult<PopulatedFile> {
    match f {
        &ParsedFile::Date(ref x) => _populate_datebased(&x),
        &ParsedFile::Season(ref x) => _populate_seasonbased(&x),
    }
}
