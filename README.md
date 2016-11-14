# Installing

Install dependencies

```
apt install cmake
```

# Running

Run with:

```
env RUST_LOG=otter=info,logger=info cargo run
```

# TODO

## Functionality

* Make wikilinks work properly
* Figure out how to display changelogs/diffs sanely (patch, unidiff crates can parse)
* Figure out sessions, user accounts.
* Figure out backlinks
* Play with Hoedown Markdown extensions/options
* Syntax highlighting, somehow???

## Utility

* Sanitize inputs, make 'em safe, however that's done. (ammonia crate?)
* Make better logging.
* Tests!

## Presentation

* Make nice templates :/
* Documentation

## Other

* Mongle/worry about license (MIT if all the dependent crates support it)
* Play with Pullmark?
