## Dandy
### A Vanity Address Generator for the Dusk Netwwork

#### Getting Started

Dandy is written in Rust and can be compiled using the Cargo tool. Note that some of the dependencies of this project rely on the nightly build features, and thus the Cargo setting will need to updated to allow this work properly.

```
rustup override set nightly
cargo build --release
```

##### Program Options

The executable takes 3 commands like arguments that are listed below.

```
dandy <prefix> <caseSensitive?> <resultFilePath>
```

prefix refers to the prefix that you desire for you vanity address. Longer prefix will require more searching and time.

caseSensitive is a boolean variable that indicates whether you want to find addreses that match the specifc casing of the prefix (more compute intensive), or not. the values that work here are `t`, `true`, `f`, and `false`.

resultFilepath is the filepath that will store all the addresses found that match your prefix. Each line of the file will contain a list of comma separated values that relate to a single address. These values are, in order: 

12 word seed phrase, address, view key, secret key