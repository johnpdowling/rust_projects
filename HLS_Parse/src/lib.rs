//! Utilites for parsing [HTTP-Live Streaming (HLS)][wiki] playlists.
//!
//! This library follows [this specification][spec] to parse [ext-m3u][m3u] formatted data.
//!
//! [m3u]: https://en.wikipedia.org/wiki/M3U#Extended_M3U
//! [spec]: https://datatracker.ietf.org/doc/html/rfc8216#section-4
//! [wiki]: https://en.wikipedia.org/wiki/HTTP_Live_Streaming

mod media_playlist;

pub use media_playlist::{MediaPlaylist, MediaSegment};
