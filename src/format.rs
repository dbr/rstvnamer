use super::populate::PopulatedFile;
use super::parsing::ParsedFile;
use tvdb::TvdbResult;
use std::path::Path;

fn ext(path: &Path) -> String {
    path.extension().map_or("".into(), |x| {
        x.to_str().map_or("".into(), |x| format!(".{}", x))
    })
}

/// Construct a new path
pub fn format(
    populated: &PopulatedFile,
    parsed: &ParsedFile,
    original: &Path,
) -> TvdbResult<String> {
    let extension = ext(original);

    let name = format!(
        "{} - [{:02}x{:02}] - {}{}",
        populated.series,
        populated.season,
        populated.episode,
        populated.episodename,
        extension
    );
    return Ok(name);
}
