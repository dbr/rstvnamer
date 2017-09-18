use regex::Regex;
use regex;
use std::path::Path;

use tvdb::Date;
use super::utils::intify;
use super::{TvnamerError, TvnamerResult};



/// Episode information prased from a filename.
#[derive(Debug)]
pub struct DateBased{
    pub series: String,
    pub date: Date,
}

#[derive(Debug)]
pub struct SeasonBased{
    pub series: String,
    pub season: u32,
    pub episode: u32
}

#[derive(Debug)]
pub enum ParsedFile{
    Date(DateBased),
    Season(SeasonBased),
}

fn load_patterns() -> TvnamerResult<Vec<Regex>>{
    let raw_patterns = vec![
        // r"(?x) # [group] Show - 01-02 [crc]
        // ^\[(?P<group>.+?)\][ ]?               # group name, captured for [#100]
        // (?P<seriesname>.*?)[ ]?[-_][ ]?          # show name, padding, spaces?
        // (?P<episodenumberstart>\d+)              # first episode number
        // ([-_]\d+)*                               # optional repeating episodes
        // [-_](?P<episodenumberend>\d+)            # last episode number
        // (?=                                      # Optional group for crc value (non-capturing)
        //   .*                                     # padding
        //   \[(?P<crc>.+?)\]                       # CRC value
        // )?                                       # End optional crc group
        // [^/]*$",

        // r"(?x) # [group] Show - 01 [crc]
        // ^\[(?P<group>.+?)\][ ]?               # group name, captured for [#100]
        // (?P<seriesname>.*)                       # show name
        // [ ]?[-_][ ]?                             # padding and seperator
        // (?P<episodenumber>\d+)                   # episode number
        // (?=                                      # Optional group for crc value (non-capturing)
        //   .*                                     # padding
        //   \[(?P<crc>.+?)\]                       # CRC value
        // )?                                       # End optional crc group
        // [^\\/]*$",

        // FIXME: Disabled because of (?P=blah ) syntax
        // r"(?x) # foo s01e23 s01e24 s01e25 *
        // ^((?P<seriesname>.+?)[ \._-])?          # show name
        // [Ss](?P<seasonnumber>[0-9]+)             # s01
        // [\.- ]?                                 # separator
        // [Ee](?P<episodenumberstart>[0-9]+)       # first e23
        // ([\.- ]+                                # separator
        // [Ss](?P=seasonnumber)                    # s01
        // [\.- ]?                                 # separator
        // [Ee][0-9]+)*                             # e24 etc (middle groups)
        // ([\.- ]+                                # separator
        // [Ss](?P=seasonnumber)                    # last s01
        // [\.- ]?                                 # separator
        // [Ee](?P<episodenumberend>[0-9]+))        # final episode number
        // [^\\/]*$",

        r"(?x) # foo.s01e23e24*
        ^((?P<seriesname>.+?)[ \._-])?          # show name
        [Ss](?P<seasonnumber>[0-9]+)             # s01
        [\.- ]?                                 # separator
        [Ee](?P<episodenumberstart>[0-9]+)       # first e23
        ([\.- ]?                                # separator
        [Ee][0-9]+)*                             # e24e25 etc
        [\.- ]?[Ee](?P<episodenumberend>[0-9]+) # final episode num
        [^/]*$",

        // FIXME: Disabled because of (?P=blah ) syntax
        // r"(?x) # foo.1x23 1x24 1x25
        // ^((?P<seriesname>.+?)[ \._-])?          # show name
        // (?P<seasonnumber>[0-9]+)                 # first season number (1)
        // [xX](?P<episodenumberstart>[0-9]+)       # first episode (x23)
        // ([ \._-]+                               # separator
        // (?P=seasonnumber)                        # more season numbers (1)
        // [xX][0-9]+)*                             # more episode numbers (x24)
        // ([ \._-]+                               # separator
        // (?P=seasonnumber)                        # last season number (1)
        // [xX](?P<episodenumberend>[0-9]+))        # last episode number (x25)
        // [^\\/]*$",

        r"(?x) # foo.1x23x24*
        ^((?P<seriesname>.+?)[ \._-])?          # show name
        (?P<seasonnumber>[0-9]+)                 # 1
        [xX](?P<episodenumberstart>[0-9]+)       # first x23
        ([xX][0-9]+)*                            # x24x25 etc
        [xX](?P<episodenumberend>[0-9]+)         # final episode num
        [^\\/]*$",

        r"(?x) # foo.s01e23-24*
        ^((?P<seriesname>.+?)[ \._-])?          # show name
        [Ss](?P<seasonnumber>[0-9]+)             # s01
        [\.- ]?                                 # separator
        [Ee](?P<episodenumberstart>[0-9]+)       # first e23
        (                                        # -24 etc
             [-]
             [Ee]?[0-9]+
        )*
             [-]                                # separator
             [Ee]?(?P<episodenumberend>[0-9]+)   # final episode num
        [\.- ]                                  # must have a separator (prevents s01e01-720p from being 720 episodes)
        [^\\/]*$",

        r"(?x) # foo.1x23-24*
        ^((?P<seriesname>.+?)[ \._-])?          # show name
        (?P<seasonnumber>[0-9]+)                 # 1
        [xX](?P<episodenumberstart>[0-9]+)       # first x23
        (                                        # -24 etc
             [-+][0-9]+
        )*
             [-+]                               # separator
             (?P<episodenumberend>[0-9]+)        # final episode num
        ([\.+ -].*                              # must have a separator (prevents 1x01-720p from being 720 episodes)
        |
        $)",

        r"(?x) # foo.[1x09-11]*
        ^(?P<seriesname>.+?)[ \._-]          # show name and padding
        \[                                       # [
            ?(?P<seasonnumber>[0-9]+)            # season
        [xX]                                     # x
            (?P<episodenumberstart>[0-9]+)       # episode
            ([-+] [0-9]+)*
        [-+]                                    # -
            (?P<episodenumberend>[0-9]+)         # episode
        \]                                       # \]
        [^\\/]*$",

        r"(?x) # foo - [012]
        ^((?P<seriesname>.+?)[ \._-])?       # show name and padding
        \[                                       # [ not optional (or too ambigious)
        (?P<episodenumber>[0-9]+)                # episode
        \]                                       # ]
        [^\\/]*$",

        r"(?x) # foo.s0101, foo.0201
        ^(?P<seriesname>.+?)[ \._-]
        [Ss](?P<seasonnumber>[0-9]{2})
        [\.- ]?
        (?P<episodenumber>[0-9]{2})
        [^0-9]*$",

        r"(?x) # foo.1x09*
        ^((?P<seriesname>.+?)[ \._-])?       # show name and padding
        \[?                                      # [ optional
        (?P<seasonnumber>[0-9]+)                 # season
        [xX]                                     # x
        (?P<episodenumber>[0-9]+)                # episode
        \]?                                      # ] optional
        [^\\/]*$",

        r"(?x) # foo.s01.e01, foo.s01_e01, foo.s01 - e01
        ^((?P<seriesname>.+?)[ \._-])?
        \[?
        [Ss](?P<seasonnumber>[0-9]+)[ ]?[\._- ]?[ ]?
        [Ee]?(?P<episodenumber>[0-9]+)
        \]?
        [^\\/]*$",

        r"(?x) # foo.2010.01.02.etc
        ^((?P<seriesname>.+?)[ \._-])?          # show name
        (?P<year>\d{4})                          # year
        [ \._-]                                 # separator
        (?P<month>\d{2})                         # month
        [ \._-]                                 # separator
        (?P<day>\d{2})                           # day
        [^\\/]*$",

        r"(?x) # foo - [01.09]
        ^((?P<seriesname>.+?))                # show name
        [ \._-]?                                # padding
        \[                                       # [
        (?P<seasonnumber>[0-9]+?)                # season
        [.]                                      # .
        (?P<episodenumber>[0-9]+?)               # episode
        \]                                       # ]
        [ \._-]?                                # padding
        [^\\/]*$",

        // FIXME: Strange error with mismatched parens?
        // r"(?x) # Foo - S2 E 02 - etc
        // ^(?P<seriesname>.+?)[ ]?[ \._-][ ]?
        // [Ss](?P<seasonnumber>[0-9]+)[\.- ]?
        // [Ee]?[ ]?(?P<episodenumber>[0-9]+)
        // [^\\/]*$",

        // FIXME: Confused syntax for [] stuff
        // r"(?x) # Show - Episode 9999 [S 12 - Ep 131] - etc
        // (?P<seriesname>.+)                       # Showname
        // [ ]-[ ]                                  # -
        // [Ee]pisode[ ]\d+                         # Episode 1234 (ignored)
        // [ ]
        // \[                                       # [
        // [sS][ ]?(?P<seasonnumber>\d+)            # s 12
        // ([ ]|[ ]-[ ]|-)                          # space, or -
        // ([eE]|[eE]p)[ ]?(?P<episodenumber>\d+)   # e or ep 12
        // \]                                       # ]
        // .*$                                      # rest of file
        // ",

        r"(?x) # show name 2 of 6 - blah
        ^(?P<seriesname>.+?)                  # Show name
        [ \._-]                                 # Padding
        (?P<episodenumber>[0-9]+)                # 2
        of                                       # of
        [ \._-]?                                # Padding
        \d+                                      # 6
        ([\._ -]|$|[^\\/]*$)                     # More padding, then anything
        ",

        r"(?x) # Show.Name.Part.1.and.Part.2
        ^(?i)
        (?P<seriesname>.+?)                        # Show name
        [ \._-]                                   # Padding
        (?:part|pt)?[\._ -]
        (?P<episodenumberstart>[0-9]+)             # Part 1
        (?:
          [ \._-](?:and|&|to)                        # and
          [ \._-](?:part|pt)?                        # Part 2
          [ \._-](?:[0-9]+))*                        # (middle group, optional, repeating)
        [ \._-](?:and|&|to)                        # and
        [ \._-]?(?:part|pt)?                       # Part 3
        [ \._-](?P<episodenumberend>[0-9]+)        # last episode number, save it
        [\._ -][^\\/]*$                            # More padding, then anything
        ",

        // FIXME: ANother paren problem
        // r"(?x) # Show.Name.Part1
        // ^(?P<seriesname>.+?)                  # Show name\n
        // [ \._-]                               # Padding\n
        // [Pp]art[ ](?P<episodenumber>[0-9]+)      # Part 1\n
        // [\\._ -][^\\/]*$                         # More padding, then anything\n
        // ",

        // FIXME: ANother paren problem
        // r"(?x) # show name Season 01 Episode 20
        // ^(?P<seriesname>.+?)[ ]?               # Show name
        // [Ss]eason[ ]?(?P<seasonnumber>[0-9]+)[ ]? # Season 1
        // [Ee]pisode[ ]?(?P<episodenumber>[0-9]+)   # Episode 20
        // [^\\/]*$                              # Anything
        // ",

        r"(?x) # foo.103*
        ^(?P<seriesname>.+)[ \._-]
        (?P<seasonnumber>[0-9]{1})
        (?P<episodenumber>[0-9]{2})
        [\._ -][^\\/]*$",

        r"(?x) # foo.0103*
        ^(?P<seriesname>.+)[ \._-]
        (?P<seasonnumber>[0-9]{2})
        (?P<episodenumber>[0-9]{2,3})
        [\._ -][^\\/]*$",

        r"(?x) # show.name.e123.abc
        ^(?P<seriesname>.+?)                  # Show name
        [ \._-]                                 # Padding
        [Ee](?P<episodenumber>[0-9]+)            # E123
        [\._ -][^\\/]*$                          # More padding, then anything
        ",

        ];

    let mut patterns: Vec<Regex> = vec![];

    for pat in raw_patterns.iter(){
        let comp = Regex::new(pat);
        match comp {
            Ok(x) => patterns.push(x),
            Err(e) => return Err(TvnamerError::InternalError{
                reason: format!("Error compiling regex: {}\n  Pattern:\n{}", e, pat)}),
        }

    }

    return Ok(patterns);
}

fn clean_series(name: String) -> String {
    name.replace(".", " ")
}

/// Parses a filename and returns a `ParsedFile`
pub fn parse(fname:&Path) -> TvnamerResult<ParsedFile>{
    /// Check a regex contains all specified named captures
    fn check_matches(cap: &regex::Captures, things: Vec<&str>) -> bool{
        let mut matches = true;
        for name in things.iter(){
            if cap.name(name).is_none(){
                matches = false;
            }
        }
        return matches;
    }

    // Load all regex patterns
    let patterns = load_patterns()?;

    let basename = Path::new(fname).file_stem()
        .ok_or(TvnamerError::InternalError{
            reason: format!("No file name found for path {:?}", fname)})?
            .to_str().ok_or(TvnamerError::InternalError{
                reason: "Failed to convert to string".into()})?;

    for pat in patterns.iter(){
        if let Some(x) = pat.captures(basename) {

            if check_matches(&x, vec!["seriesname", "seasonnumber", "episodenumber"]) {
                return Ok(ParsedFile::Season(SeasonBased{
                    series: clean_series(x.name("seriesname").unwrap().to_owned()),
                    season: intify(x.name("seasonnumber").unwrap()),
                    episode: intify(x.name("episodenumber").unwrap()),
                }));

            } else if check_matches(&x, vec!["seriesname", "year", "month", "day"]) {
                return Ok(ParsedFile::Date(DateBased{
                    series: clean_series(x.name("seriesname").unwrap().to_owned()),
                    date: Date{
                        year: intify(x.name("year").unwrap()),
                        month: intify(x.name("month").unwrap()),
                        day: intify(x.name("day").unwrap()),
                    },
                }));

            } else {
                // Unhandled capture groups, throw error
                let mut names: Vec<String> = vec![];
                for (name, _) in x.iter_named(){
                    names.push(name.to_owned());
                }
                return Err(TvnamerError::ParseError{reason:
                    format!("Unhandled capture groups {:?} in pattern {}", names, pat)});
            }
        }
    }

    // TODO: Patterns which match against full path
    return Err(TvnamerError::ParseError{reason:
        format!("Unrecognised file name {} matched no patterns", basename)});
}
