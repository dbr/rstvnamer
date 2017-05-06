use super::populate::PopulatedFile;
use tvdb::{TvdbError, TvdbResult};

pub fn format(pf: &PopulatedFile) -> TvdbResult<String>{
    let name = format!("{} - [{:02}x{:02}] - {}",
                       pf.series,
                       pf.season,
                       pf.episode,
                       pf.episodename);
    return Ok(name);
}
