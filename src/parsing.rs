use super::Date;
use regex::Regex;



/// Episode information prased from a filename.
#[derive(Debug)]
pub struct DateBased{
    pub series: String,
    pub date: Date,
}

#[derive(Debug)]
pub struct SeasonBased{
    pub series: String,
    pub season: i32,
    pub episode: i32
}

#[derive(Debug)]
pub enum ParsedFile{
    Date(DateBased),
    Season(SeasonBased),
}

fn load_patterns() -> Vec<Regex>{
    let raw_patterns = vec![
        r"(?x) # [group] Show - 01-02 [crc]
        ^\[(?P<group>.+?)\][ ]?               # group name, captured for [#100]
        (?P<seriesname>.*?)[ ]?[-_][ ]?          # show name, padding, spaces?
        (?P<episodenumberstart>\d+)              # first episode number
        ([-_]\d+)*                               # optional repeating episodes
        [-_](?P<episodenumberend>\d+)            # last episode number
        (?=                                      # Optional group for crc value (non-capturing)
          .*                                     # padding
          \[(?P<crc>.+?)\]                       # CRC value
        )?                                       # End optional crc group
        [^\/]*$",

        r"(?x) # [group] Show - 01 [crc]
        ^\[(?P<group>.+?)\][ ]?               # group name, captured for [#100]
        (?P<seriesname>.*)                       # show name
        [ ]?[-_][ ]?                             # padding and seperator
        (?P<episodenumber>\d+)                   # episode number
        (?=                                      # Optional group for crc value (non-capturing)
          .*                                     # padding
          \[(?P<crc>.+?)\]                       # CRC value
        )?                                       # End optional crc group
        [^\/]*$",

        r"(?x) # foo s01e23 s01e24 s01e25 *
        ^((?P<seriesname>.+?)[ \._\-])?          # show name
        [Ss](?P<seasonnumber>[0-9]+)             # s01
        [\.\- ]?                                 # separator
        [Ee](?P<episodenumberstart>[0-9]+)       # first e23
        ([\.\- ]+                                # separator
        [Ss](?P=seasonnumber)                    # s01
        [\.\- ]?                                 # separator
        [Ee][0-9]+)*                             # e24 etc (middle groups)
        ([\.\- ]+                                # separator
        [Ss](?P=seasonnumber)                    # last s01
        [\.\- ]?                                 # separator
        [Ee](?P<episodenumberend>[0-9]+))        # final episode number
        [^\/]*$",

        r"(?x) # foo.s01e23e24*
        ^((?P<seriesname>.+?)[ \._\-])?          # show name
        [Ss](?P<seasonnumber>[0-9]+)             # s01
        [\.\- ]?                                 # separator
        [Ee](?P<episodenumberstart>[0-9]+)       # first e23
        ([\.\- ]?                                # separator
        [Ee][0-9]+)*                             # e24e25 etc
        [\.\- ]?[Ee](?P<episodenumberend>[0-9]+) # final episode num
        [^\/]*$",

        r"(?x) # foo.1x23 1x24 1x25
        ^((?P<seriesname>.+?)[ \._\-])?          # show name
        (?P<seasonnumber>[0-9]+)                 # first season number (1)
        [xX](?P<episodenumberstart>[0-9]+)       # first episode (x23)
        ([ \._\-]+                               # separator
        (?P=seasonnumber)                        # more season numbers (1)
        [xX][0-9]+)*                             # more episode numbers (x24)
        ([ \._\-]+                               # separator
        (?P=seasonnumber)                        # last season number (1)
        [xX](?P<episodenumberend>[0-9]+))        # last episode number (x25)
        [^\/]*$",

        // foo.1x23x24*
        r"(?x)
        ^((?P<seriesname>.+?)[ \._\-])?          # show name
        (?P<seasonnumber>[0-9]+)                 # 1
        [xX](?P<episodenumberstart>[0-9]+)       # first x23
        ([xX][0-9]+)*                            # x24x25 etc
        [xX](?P<episodenumberend>[0-9]+)         # final episode num
        [^\/]*$",

        // foo.s01e23-24*
        r"(?x)
        ^((?P<seriesname>.+?)[ \._\-])?          # show name
        [Ss](?P<seasonnumber>[0-9]+)             # s01
        [\.\- ]?                                 # separator
        [Ee](?P<episodenumberstart>[0-9]+)       # first e23
        (                                        # -24 etc
             [\-]
             [Ee]?[0-9]+
        )*
             [\-]                                # separator
             [Ee]?(?P<episodenumberend>[0-9]+)   # final episode num
        [\.\- ]                                  # must have a separator (prevents s01e01-720p from being 720 episodes)
        [^\/]*$",

        // foo.1x23-24*
        r"(?x)
        ^((?P<seriesname>.+?)[ \._\-])?          # show name
        (?P<seasonnumber>[0-9]+)                 # 1
        [xX](?P<episodenumberstart>[0-9]+)       # first x23
        (                                        # -24 etc
             [\-+][0-9]+
        )*
             [\-+]                               # separator
             (?P<episodenumberend>[0-9]+)        # final episode num
        ([\.\-+ ].*                              # must have a separator (prevents 1x01-720p from being 720 episodes)
        |
        $)",

        // foo.[1x09-11]*
        r"(?x)
        ^(?P<seriesname>.+?)[ \._\-]          # show name and padding
        \[                                       # [
            ?(?P<seasonnumber>[0-9]+)            # season
        [xX]                                     # x
            (?P<episodenumberstart>[0-9]+)       # episode
            ([\-+] [0-9]+)*
        [\-+]                                    # -
            (?P<episodenumberend>[0-9]+)         # episode
        \]                                       # \]
        [^\\/]*$",

        // foo - [012]
        r"(?x)
        ^((?P<seriesname>.+?)[ \._\-])?       # show name and padding
        \[                                       # [ not optional (or too ambigious)
        (?P<episodenumber>[0-9]+)                # episode
        \]                                       # ]
        [^\\/]*$",

        // foo.s0101, foo.0201
        r"(?x)
        ^(?P<seriesname>.+?)[ \._\-]
        [Ss](?P<seasonnumber>[0-9]{2})
        [\.\- ]?
        (?P<episodenumber>[0-9]{2})
        [^0-9]*$",

        // foo.1x09*
        r"(?x)
        ^((?P<seriesname>.+?)[ \._\-])?       # show name and padding
        \[?                                      # [ optional
        (?P<seasonnumber>[0-9]+)                 # season
        [xX]                                     # x
        (?P<episodenumber>[0-9]+)                # episode
        \]?                                      # ] optional
        [^\\/]*$",

        // foo.s01.e01, foo.s01_e01, "foo.s01 - e01"
        r"(?x)
        ^((?P<seriesname>.+?)[ \._\-])?
        \[?
        [Ss](?P<seasonnumber>[0-9]+)[ ]?[\._\- ]?[ ]?
        [Ee]?(?P<episodenumber>[0-9]+)
        \]?
        [^\\/]*$",

        // foo.2010.01.02.etc
        r"(?x)
        ^((?P<seriesname>.+?)[ \._\-])?          # show name
        (?P<year>\d{4})                          # year
        [ \._\-]                                 # separator
        (?P<month>\d{2})                         # month
        [ \._\-]                                 # separator
        (?P<day>\d{2})                           # day
        [^\/]*$",

        // foo - [01.09]
        r"(?x)
        ^((?P<seriesname>.+?))                # show name
        [ \._\-]?                                # padding
        \[                                       # [
        (?P<seasonnumber>[0-9]+?)                # season
        [.]                                      # .
        (?P<episodenumber>[0-9]+?)               # episode
        \]                                       # ]
        [ \._\-]?                                # padding
        [^\\/]*$",

        // Foo - S2 E 02 - etc
        r"(?x)
        ^(?P<seriesname>.+?)[ ]?[ \._\-][ ]?
        [Ss](?P<seasonnumber>[0-9]+)[\.\- ]?
        [Ee]?[ ]?(?P<episodenumber>[0-9]+)
        [^\\/]*$",

        // Show - Episode 9999 [S 12 - Ep 131] - etc
        r"(?x)
        (?P<seriesname>.+)                       # Showname
        [ ]-[ ]                                  # -
        [Ee]pisode[ ]\d+                         # Episode 1234 (ignored)
        [ ]
        \[                                       # [
        [sS][ ]?(?P<seasonnumber>\d+)            # s 12
        ([ ]|[ ]-[ ]|-)                          # space, or -
        ([eE]|[eE]p)[ ]?(?P<episodenumber>\d+)   # e or ep 12
        \]                                       # ]
        .*$                                      # rest of file
        ",

        // show name 2 of 6 - blah
        r"(?x)
        ^(?P<seriesname>.+?)                  # Show name
        [ \._\-]                                 # Padding
        (?P<episodenumber>[0-9]+)                # 2
        of                                       # of
        [ \._\-]?                                # Padding
        \d+                                      # 6
        ([\._ -]|$|[^\\/]*$)                     # More padding, then anything
        ",

        // Show.Name.Part.1.and.Part.2
        r"(?x)
        ^(?i)
        (?P<seriesname>.+?)                        # Show name
        [ \._\-]                                   # Padding
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

        // Show.Name.Part1
        r"(?x)
        ^(?P<seriesname>.+?)                  # Show name\n
        [ \\._\\-]                               # Padding\n
        [Pp]art[ ](?P<episodenumber>[0-9]+)      # Part 1\n
        [\\._ -][^\\/]*$                         # More padding, then anything\n
        ",

        // show name Season 01 Episode 20
        r"(?x)
        ^(?P<seriesname>.+?)[ ]?               # Show name
        [Ss]eason[ ]?(?P<seasonnumber>[0-9]+)[ ]? # Season 1
        [Ee]pisode[ ]?(?P<episodenumber>[0-9]+)   # Episode 20
        [^\\/]*$                              # Anything
        ",

        // foo.103*
        r"(?x)
        ^(?P<seriesname>.+)[ \._\-]
        (?P<seasonnumber>[0-9]{1})
        (?P<episodenumber>[0-9]{2})
        [\._ -][^\\/]*$",

        // foo.0103*
        r"(?x)
        ^(?P<seriesname>.+)[ \._\-]
        (?P<seasonnumber>[0-9]{2})
        (?P<episodenumber>[0-9]{2,3})
        [\._ -][^\\/]*$",

        // show.name.e123.abc
        r"(?x)
        ^(?P<seriesname>.+?)                  # Show name
        [ \._\-]                                 # Padding
        [Ee](?P<episodenumber>[0-9]+)            # E123
        [\._ -][^\\/]*$                          # More padding, then anything
        ",

        ];

    let mut patterns: Vec<Regex> = vec![];

    for pat in raw_patterns.iter(){
        let comp = Regex::new(pat);
        match comp {
            Ok(x) => patterns.push(x),
            Err(e) => println!("Error while parsing:\n{}\nPattern:\n{}", pat, e),
        }
        
    }

    return patterns;
}

/// Parses a filename and returns a ParsedFile
pub fn parse(fname:&str) -> Option<ParsedFile>{
    let patterns = load_patterns();

    for pat in patterns.iter(){
        let caps = pat.captures(fname);
        if let Some(x) = caps {
            println!("Pow");
        }
    }
    
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
