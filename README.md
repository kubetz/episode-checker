# Episode Checker

This is a simple CLI utility that traverses a given directory structure to look for (completely legally obtained) TV shows. For each found show it will check if there are any new episodes available and print them out into the standard output. Downloading found episodes is up to the user.

## Details

The tool walks through the directories recursively. Each directory is treated as a possible TV show name. If the directory includes any file that contains season and episode number information (e.g. `S01E01`), it is considered a valid candidate. All files are parsed in the directory to find the latest (legally!) downloaded episode.

Episode checker will then try to find the show using [TVmaze](https://www.tvmaze.com/) API. and check if there are any new episodes that have already aired and could be (legally!) retrieved. If there are, they will be printed out on the standard output.

If the show candidate cannot be found on TVmaze, it will be skipped silently. Same thing will happen if the highest found episode is not available there. This is done without panicking as there is a likely chance to find directories and file names that are false positives and don't really represent real TV show data.

If API data is successfully retrieved, but parsing fails, the tool will basically explode as that means something broke and someone should be blamed - most likely TVmaze staff 😏.

Example directory structure:

```
├── The First of Them
│   ├── The.First.of.Them.S01E01.WEB.x264-LEGALSOURCE.mkv
│   ├── The.First.of.Them.S01E02.WEB.x264-LEGALSOURCE.mkv
│   ├── The.First.of.Them.S01E03.WEB.x264-LEGALSOURCE.mkv
├── Random Subdirectory
│   ├── The Eagle and The Summer Cadet
│   │   ├── The.Eagle.and.The.Summer.Cadet.S01E01.720p.WEBRip.x264-ALSOLEGAL.mkv
│   │   ├── The.Eagle.and.The.Summer.Cadet.S01E02.720p.WEBRip.x264-ALSOLEGAL.mkv
│   │   ├── The.Eagle.and.The.Summer.Cadet.S01E03.720p.WEBRip.x264-ALSOLEGAL.mkv
│   │   ├── The.Eagle.and.The.Summer.Cadet.S01E04.720p.WEBRip.x264-ALSOLEGAL.mkv
│   │   ├── The.Eagle.and.The.Summer.Cadet.S01E05.720p.WEBRip.x264-ALSOLEGAL.mkv
└───└───└── The.Eagle.and.The.Summer.Cadet.S01E06.720p.WEBRip.x264-ALSOLEGAL.mkv
```

In the example above, the tool will check what was the air date of S01E03 of the show `The First of Them` and check if there are any new episodes that have already aired. It will do the same for S01E06 of the `The Eagle and The Summer Cadet` show.

## Purpose

Purpose of this project is to work on my Rust skills. The chances that anyone will find it useful are extremely low, but sharing it "just in case" hopefully won't hurt anyone.

It initially started as a simple Python script. One of the first things a baby Rustacean learns is to always go for "Rewrite It In Rust"™  and this is the result of that.

I tried to go for something more robust than a simple top-down block of code. The implementation includes:
- Parsing of CLI arguments using [clap](https://docs.rs/clap/latest/clap/).
- Using HTTP client [ureq](https://docs.rs/ureq/latest/ureq/). That is more lightweight than [reqwest](https://docs.rs/reqwest/latest/reqwest/) + [tokio](https://docs.rs/tokio/latest/tokio/).
- Deserializing JSON responses using [serde](https://docs.rs/serde/latest/serde/) including a custom deserializer.
- Parsing text using [nom](https://docs.rs/nom/latest/nom/).
- Error handling using [thiserror](https://docs.rs/thiserror/latest/thiserror/).
- Other stuff: callbacks, generics, iterators, file system access, etc.
- Oh, and comments, a whole lot of comments.  

## Prerequisites

Having a Rust toolchain installed can come in pretty handy I guess. You can get it from [here](https://www.rust-lang.org/tools/install).

## Building

To build this project you need to just execute `cargo build --release` in the root directory.

If you want to install it, executing `cargo install --path .` should do the trick.

## Usage

This utility provides currently 2 positional CLI arguments that are optional.

```
Usage: episode-checker [DIR] [DIFF]

Arguments:
  [DIR]   Directory from which shows will be checked recursively for new episodes
  [DIFF]  Number of days from today representing the target airdate (default: -1)

Options:
  -h, --help  Print helpUSAGE:
```

`[DIR]` specifies the directory structure that will be the start of the recursive search. If not specified, the current working directory will be used.

`[DIFF]` specifies the number of days from today that will be used as the target airdate. If not specified, the default value of `-1` will be used. As you probably figured out `-1` is yesterday and that means the show should be already available to be (legally!) acquired. But if you want to check what was aired until one week ago, go for it and use `-- -7` (negative values are entered in a funky way, no reason to be alarmed). If you for whatever reason want to see what will be available in a month, go for `30`. You got the idea by now.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.