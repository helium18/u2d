# u2d
Uptodown Scraper (Project is still in alpha!)

---

Scrape download links from Uptodown. Returns them as pretty-printed JSON.

![](https://i.imgur.com/LXb0vJ0.gif)

## Syntax
```
u2d <link>
```
Links need to have this format : https://spotify.en.uptodown.com/windows/. The Link needs to point to the homepage.

Invalid links like this : https://spotify.en.uptodown.com/windows/download 

return an empty JSON. This functionality would be swapped for a more verbose and detailed error sometime soon.

## Download
Head to the [releases](https://github.com/helium18/u2d/releases) page and grab the binaries or [build it!](#Building)

## Building
Install [Rust](https://www.rust-lang.org/tools/install). Then execute then follow these instructions.

```
git clone https://github.com/helium18/u2d.git
cd u2d
cargo build --release 
cd target/release
```

You should be able to find the binary there. Append it to `PATH` to launch it directly from your shell or move it to a suitable location for easy access.

## Found a bug or have a suggestion?
The project is still in Alpha, bugs are likely. Head over to the [issues](https://github.com/helium18/u2d/issues) page and create an issue.

