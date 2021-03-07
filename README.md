# i3blocks_memory

This is a memory block for i3blocks written as beginner project in Rust.
It shows your memory as `used/total` and in percent in a fixed length output:

```
4.21GB/16.28GB (25.85%)
```

### Building

Use cargo to build the project.

```
cargo build --release
```

The executable is now located in `./target/release/i3blocks_memory`

## Semantic Versioning

This repository uses [SemVer](https://semver.org/) for its release
cycle.

## Branching Model

This repository uses the
[git-flow](https://danielkummer.github.io/git-flow-cheatsheet/index.html)
branching model by [Vincent Driessen](https://nvie.com/about/).
It has two branches with infinite lifetime:

* [master](https://github.com/MichaelSasser/i3blocks_memory/tree/master)
* [develop](https://github.com/MichaelSasser/i3blocks_memory/tree/develop)

The master branch gets updated on every release. The develop branch is the
merging branch.

## License
Copyright &copy; 2020-2021 Michael Sasser <Info@MichaelSasser.org>. 
Released under the GPLv3 license.
