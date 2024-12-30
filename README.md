# Twemoji Inlining

Inline twitter emojis with `svg` elements or `img` elements into your HTML templates.

**Example:**

```rs
use twemoji::TwemojiParser;
use std::path::PathBuf;

let mut emoji_parser =
    TwemojiParser::inline_from_local_file(PathBuf::from("./emoji"));
dbg!(emoji_parser.parse("Hello 😀"));
```

This will replace the 😀 with a `svg` HTML element, assuming the SVG files are in the `./emoji` directory.


You can also use link to a URL:

```rs
use twemoji::TwemojiParser;
use std::path::PathBuf;

let mut emoji_parser = TwemojiParser::link_from_url(
    PathBuf::from_str("/assets/emoji/svg").unwrap(),
    "svg".to_string(),
);
deb!(emoji_parser.parse("Hello 😀"));
```

This will replace the 😀 with a `img` HTML element, linking to the `/assets/emoji/svg/<code_point>.svg` endpoint.
