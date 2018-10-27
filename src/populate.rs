extern crate tvdb;
use super::parsing::{DateBased, ParsedFile, SeasonBased, Date};
use super::config::TVDB_API_KEY;
use super::errors::{TvnamerResult, TvnamerError};

use tvdb::TvdbError;

/// Episode with complete set of information, usually expanded from a
/// ParsedFile instance
#[derive(Debug)]
pub struct PopulatedFile {
    pub series: String,
    pub season: u32,
    pub episode: u32,
    pub episodename: String,
    pub airdate: String,
}

fn find_episode<'a, 'b>(ep_data: &'a Vec<tvdb::data::BasicEpisode>, file: &'b SeasonBased) -> TvnamerResult<&'a tvdb::data::BasicEpisode>{
    for ep in ep_data.iter() {
        let x = ep;
        if x.aired_season == Some(file.season) && x.aired_episode_number == Some(file.episode) {
            return Ok(ep);
        }
    }
    return Err(TvnamerError::EpisodeNotFound{what: format!("'{}' season {} episode {}", file.series, file.season, file.episode)});
}

fn _populate_seasonbased(file: &SeasonBased) -> TvnamerResult<PopulatedFile> {
    let api = tvdb::Tvdb::new(TVDB_API_KEY);
    api.login()?;
    let sr = api.search(Some(&file.series), None)?;
    //if sr.data.unwrap().is_empty() {
    //    return Err(TvdbError::SeriesNotFound.into());
    //}

    let sr_data = &sr.data.unwrap()[0];
    let sn = &sr_data.series_name;;
    let ep = api.series_episodes(sr_data.id.unwrap(), 0)?;
    let ep_data = ep.data.unwrap();
    let matched = find_episode(&ep_data, &file)?;

    let pf = PopulatedFile {
        series: sn.clone(),
        season: file.season,
        episode: file.episode,
        episodename: matched.clone().episode_name.unwrap(),
        airdate: matched.clone().first_aired.unwrap(),
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
