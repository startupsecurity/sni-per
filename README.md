# SNI-PER

| 🚨 THIS PROJECT IS IN PROGRESS. Create issues if you have any ideas or suggestions.

TL;DR download and query assets found in certificates from local sqlite database

## Installation

```bash
cargo install sni-per
```

## Caveats

* `sni-per sync` will download the latest sni ranges for all providers and save them to `~/.sni-per/tmp`. In the future this will be configurable


## Notes

* Pulling all data from https://github.com/lord-alfred/ipranges