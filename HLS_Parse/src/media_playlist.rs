//! Utilites for parsing media playlists (i.e. not master playlists).

use core::time::Duration;

use anyhow::Result;

/// RFC8216, Section 4 tag constants
static HEADER_TAG: &'static str = "#EXTM3U";
static VERSION_TAG: &'static str = "#EXT-X-VERSION";
static VERSION_PRE: &'static str = "#EXT-X-VERSION:";
static ENDLIST_TAG: &'static str = "#EXT-X-ENDLIST";
static DURATION_TAG: &'static str = "#EXT-X-TARGETDURATION";
static DURATION_PRE: &'static str = "#EXT-X-TARGETDURATION:";
static SEGMENT_TAG: &'static str = "#EXTINF";
static SEGMENT_PRE: &'static str = "#EXTINF:";
static BYTERANGE_TAG: &'static str = "#EXT-X-BYTERANGE";

/// Storage for HLS Media Playlist data. Can be constructed from `ext-m3u` data using
/// [`parse_ext_m3u`][MediaPlaylist::parse_ext_m3u].
#[derive(Debug, Clone, PartialEq)]
pub struct MediaPlaylist {
    /// Whether or not an ENDLIST tag was found. See
    /// <https://datatracker.ietf.org/doc/html/rfc8216#section-4.3.3.4>.
    ended: bool,

    segments: Vec<MediaSegment>,

    /// Duration that no media segment can exceed. See
    /// <https://datatracker.ietf.org/doc/html/rfc8216#section-4.3.3.1>.
    target_duration: Duration,

    /// Version of playlist for compatibility. See
    /// <https://datatracker.ietf.org/doc/html/rfc8216#section-4.3.1.2>.
    version: u64,
}

/// A media segment contains information to actually load the presentation. See [the
/// specification][spec] for more details.
///
/// [spec]: https://datatracker.ietf.org/doc/html/rfc8216#section-3
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaSegment {
    /// From the #EXTINF tag. See <https://datatracker.ietf.org/doc/html/rfc8216#section-4.3.2.1>.
    duration: Duration,

    /// Relative URL of media segment. See
    /// <https://datatracker.ietf.org/doc/html/rfc8216#section-4.3.2> and
    /// <https://datatracker.ietf.org/doc/html/rfc8216#section-4.1>.
    url: String,
}

impl MediaPlaylist {
    /// Parses the given file into a [`MediaPlaylist`], returning an error if the file does not
    /// adhere to the specification.
    pub fn parse_ext_m3u(_file: &str) -> Result<Self> {
        let lines: Vec<&str> = _file.lines().collect();
        
        //return error if our input contains no data
        if lines.len() == 0 { return Err(anyhow::Error::msg("Input contains no data")); }
        
        //RFC8216 4.3.1.1 requirement
        if lines[0] != HEADER_TAG { return Err(anyhow::Error::msg("Input doesn't start with EXTM3U tag")); }
        
        //RFC8216 4.3.1.2 requirements
        let mut file_version: u64 = 0;
        if lines.iter().filter(|x| x.starts_with(VERSION_TAG)).count() > 1 {
            return Err(anyhow::Error::msg("Playlist contains more than 1 version tag"));
        }
        let version_line: Option<&str> = lines.iter().find(|x| x.starts_with(VERSION_TAG)).copied();
        if version_line != None
        {
            let version_string: Option<&str> = version_line.unwrap().strip_prefix(VERSION_PRE);
            if version_string != None {
                let parsed_version = version_string.unwrap().parse::<u64>();
                match parsed_version {
                    Ok(..) => file_version = parsed_version.unwrap(),
                    Err(..) => return Err(anyhow::Error::msg("Version tag found, but could not parse"))
                }
            }
            else { return Err(anyhow::Error::msg("Version tag found, but could not parse")); }
        }
        else {
            //no version tag, a rigorous check makes sure we only have V1 tags 
        }
        
        //RFC8216 4.3.3.1 requirements
        let mut file_duration: Duration = Duration::new(0, 0);
        if lines.iter().filter(|x| x.starts_with(DURATION_TAG)).count() > 1 {
            return Err(anyhow::Error::msg("Playlist contains more than 1 duration tag"));
        }
        let duration_line: Option<&str> = lines.iter().find(|x| x.starts_with(DURATION_TAG)).copied();
        if duration_line != None
        {
            let duration_string: Option<&str> = duration_line.unwrap().strip_prefix(DURATION_PRE);
            if duration_string != None {
                let parsed_duration = duration_string.unwrap().parse::<u64>();
                match parsed_duration {
                    Ok(..) => file_duration = Duration::new(parsed_duration.unwrap(), 0),
                    Err(..) => return Err(anyhow::Error::msg("Duration tag found, but could not parse"))
                }
            }
        }
        else {
            return Err(anyhow::Error::msg("Duration tag not found"));
        }

        //RFC8216 4.3.2 requirements
        let mut file_segments: Vec<MediaSegment> = Vec::<MediaSegment>::new();        
        let mut new_segment: bool = false;
        let mut segment_duration = Duration::new(0,0);
        for line in lines.iter() {
            if line.starts_with(SEGMENT_TAG){
                new_segment = true;
                let info_string: Option<&str> = line.strip_prefix(SEGMENT_PRE);
                if info_string != None {
                    let info: Vec<&str> = info_string.unwrap().split(",").collect();
                    let parsed_duration = info[0].parse::<f32>();
                    match parsed_duration {
                        Ok(..) => {
                            segment_duration = Duration::from_secs_f32(parsed_duration.unwrap())
                        },
                        Err(..) => return Err(anyhow::Error::msg("Segment tag found, but could not parse duration"))
                    }
                }
            }
            else if line.starts_with(BYTERANGE_TAG)
            {
                //ignore for now
            }
            else if line.starts_with(ENDLIST_TAG)
            {
                //ignore
            }
            else if new_segment {
                //we have a url!
                file_segments.push(MediaSegment { duration: segment_duration, url: line.to_string() });
                new_segment = false;
            }
        }

        Ok(Self {
            ended: lines.iter().any(|x| x == &ENDLIST_TAG),
            segments: file_segments,
            target_duration: file_duration,
            version: file_version
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod big_buck_bunny {
        use super::*;

        // Helper because this playlist is valid and should parse correctly.
        fn big_buck_bunny() -> MediaPlaylist {
            const BIG_BUCK_BUNNY: &str = indoc::indoc! {"
                #EXTM3U
                #EXT-X-VERSION:4
                #EXT-X-ALLOW-CACHE:NO
                #EXT-X-TARGETDURATION:20
                #EXT-X-MEDIA-SEQUENCE:1
                #EXT-X-PROGRAM-DATE-TIME:2015-08-25T01:59:23.708+00:00
                #EXTINF:12.166,
                #EXT-X-BYTERANGE:1430680@4048392
                segment_1440468394459_1440468394459_1.ts
                #EXTINF:13.292,
                #EXT-X-BYTERANGE:840360@5479072
                segment_1440468394459_1440468394459_1.ts
                #EXTINF:10.500,
                #EXT-X-BYTERANGE:1009184@6319432
                segment_1440468394459_1440468394459_1.ts
                #EXTINF:11.417,
                #EXT-X-BYTERANGE:806332@0
                segment_1440468394459_1440468394459_2.ts
                #EXTINF:12.459,
                #EXT-X-BYTERANGE:701616@806332
                segment_1440468394459_1440468394459_2.ts
                #EXTINF:14.000,
                #EXT-X-BYTERANGE:931352@1507948
                segment_1440468394459_1440468394459_2.ts
                #EXTINF:19.292,
                #EXT-X-BYTERANGE:1593676@2439300
                segment_1440468394459_1440468394459_2.ts
                #EXTINF:7.834,
                #EXT-X-BYTERANGE:657812@4032976
                segment_1440468394459_1440468394459_2.ts
                #EXT-X-ENDLIST
            "};

            MediaPlaylist::parse_ext_m3u(BIG_BUCK_BUNNY).expect("Big Buck Bunny should parse")
        }

        //#[ignore = "uncomment when ready"]
        #[test]
        fn parses_version() {
            let playlist = big_buck_bunny();
            assert_eq!(playlist.version, 4);
        }

        //#[ignore = "uncomment when ready"]
        #[test]
        fn parses_target_duration() {
            let playlist = big_buck_bunny();
            assert_eq!(playlist.target_duration, Duration::from_secs(20));
        }

        //#[ignore = "uncomment when ready"]
        #[test]
        fn parses_end_tag() {
            let playlist = big_buck_bunny();
            assert!(playlist.ended);
        }

        //#[ignore = "uncomment when ready"]
        #[test]
        fn parses_segments() {
            let playlist = big_buck_bunny();
            let expected = vec![
                MediaSegment {
                    duration: Duration::from_secs_f32(12.166),
                    url: "segment_1440468394459_1440468394459_1.ts".to_string(),
                },
                MediaSegment {
                    duration: Duration::from_secs_f32(13.292),
                    url: "segment_1440468394459_1440468394459_1.ts".to_string(),
                },
                MediaSegment {
                    duration: Duration::from_secs_f32(10.500),
                    url: "segment_1440468394459_1440468394459_1.ts".to_string(),
                },
                MediaSegment {
                    duration: Duration::from_secs_f32(11.417),
                    url: "segment_1440468394459_1440468394459_2.ts".to_string(),
                },
                MediaSegment {
                    duration: Duration::from_secs_f32(12.459),
                    url: "segment_1440468394459_1440468394459_2.ts".to_string(),
                },
                MediaSegment {
                    duration: Duration::from_secs_f32(14.000),
                    url: "segment_1440468394459_1440468394459_2.ts".to_string(),
                },
                MediaSegment {
                    duration: Duration::from_secs_f32(19.292),
                    url: "segment_1440468394459_1440468394459_2.ts".to_string(),
                },
                MediaSegment {
                    duration: Duration::from_secs_f32(7.834),
                    url: "segment_1440468394459_1440468394459_2.ts".to_string(),
                },
            ];

            // Slightly easier to read failures if we go one at a time.
            assert_eq!(playlist.segments.len(), expected.len());
            for (actual, expected) in playlist.segments.into_iter().zip(expected) {
                assert_eq!(actual, expected);
            }
        }
    }
}
