# HLS Parsing Challenge

## HLS

> HTTP Live Streaming (also known as HLS) is an [HTTP][http]-based [adaptive
> bitrate streaming][abr] communications protocol developed by [Apple
> Inc.][apple] and released in 2009. Support for the protocol is widespread in
> media players, web browsers, mobile devices, and streaming media servers. As
> of 2019, an annual video industry survey has consistently found it to be the
> most popular streaming format.
>
> -- <cite>[Wikipedia][wiki]</cite>

The HLS protocol breaks down a stream into a series of small media files which are accessible via HTTP. These files are downloaded sequentially and played in order to stream the entire presentation. This protocol is defined for players using [ext-m3u][m3u] format. This file, which has an .m3u8 extension (i.e. utf-8 encoded), defines the locations of all of the media files that need to be downloaded as well as metadata about the stream. A specification for this format is available [here][spec].

## Challenge

Implement an API which can parse an m3u8 file to pass the test suite. Sample m3u8 is for [Big Buck Bunny][big_buck_bunny].

[abr]: https://en.wikipedia.org/wiki/Adaptive_bitrate_streaming
[apple]: https://en.wikipedia.org/wiki/Apple_Inc.
[big_buck_bunny]: https://docs.evostream.com/sample_content/assets/hls-bunny-rangerequest/bunny/playlist.m3u8
[http]: https://en.wikipedia.org/wiki/HTTP
[m3u]: https://en.wikipedia.org/wiki/M3U#Extended_M3U
[spec]: https://datatracker.ietf.org/doc/html/rfc8216#section-4
[wiki]: https://en.wikipedia.org/wiki/HTTP_Live_Streaming
