use super::populate::PopulatedFile;
use tvdb::{TvdbError, TvdbResult};

pub fn format(pf: PopulatedFile) -> TvdbResult<String>{
    let name = format!("{} - [{}x{}] - {}",
                       pf.series,
                       pf.season,
                       pf.episode,
                       pf.episodename);
    return Ok(name);
}
