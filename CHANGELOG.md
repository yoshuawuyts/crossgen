## 2018-08-30, Version 0.4.0
### Commits
- [[`3dc450d0cd`](https://github.com/yoshuawuyts/crossgen/commit/3dc450d0cdff1c1b381be24e9c6e544241788ef4)] (cargo-release) version 0.4.0 (Yoshua Wuyts)
- [[`dd207e51a5`](https://github.com/yoshuawuyts/crossgen/commit/dd207e51a52a8339d653752a3a0a08b2e78faf25)] warning messages on wrong access (Yoshua Wuyts)
- [[`5f0b0ef036`](https://github.com/yoshuawuyts/crossgen/commit/5f0b0ef036f7a0d518bc3d3837a45a13a4f906eb)] re-test key encryption (Yoshua Wuyts)

### Stats
```diff
 .travis.yml   |  8 ++++----
 CHANGELOG.md  | 18 ++++++++++++++++++
 Cargo.toml    |  5 ++++-
 src/lib.rs    |  1 +
 src/main.rs   | 15 +++++++++++++++
 src/travis.rs | 28 +++++++++++++---------------
 6 files changed, 55 insertions(+), 20 deletions(-)
```


## 2018-08-30, Version 0.3.0
### Commits
- [[`b3e20f2a93`](https://github.com/yoshuawuyts/crossgen/commit/b3e20f2a93c903e051bbaa10cab4d4ecbcbaca2d)] (cargo-release) version 0.3.0 (Yoshua Wuyts)
- [[`f595065e75`](https://github.com/yoshuawuyts/crossgen/commit/f595065e75e421d1faf606cb5a29356cd1a29882)] fix github token creation (Yoshua Wuyts)
- [[`bfc40598b3`](https://github.com/yoshuawuyts/crossgen/commit/bfc40598b3f979e6c0511b1d28b9cb975f7e6c8c)] update changelog (Yoshua Wuyts)

### Stats
```diff
 .travis.yml         |  2 +-
 CHANGELOG.md        | 23 +++++++++++++++++++++++
 Cargo.toml          |  6 +++---
 scripts/install.sh  |  4 ----
 src/github.rs       |  6 ++++--
 templates/script.sh |  2 +-
 6 files changed, 32 insertions(+), 11 deletions(-)
```


## 2018-08-28, Version 0.2.0
### Commits
- [[`5066d267d7`](https://github.com/yoshuawuyts/crossgen/commits/5066d267d70836d5652e9ee3b8eabc476ef05896)] (cargo-release) version 0.2.0 (Yoshua Wuyts)
- [[`edfaca59d0`](https://github.com/yoshuawuyts/crossgen/commits/edfaca59d08654e5536c5ba18834e564dc680277)] GitHub keysign (#6) (Yoshua Wuyts)
- [[`7d84a0201a`](https://github.com/yoshuawuyts/crossgen/commits/7d84a0201a77b325e09d2a70296fd5f75684cf7e)] Fix install command (#5) (Dan Reeves)

### Stats
```diff
 .travis.yml        |  8 ++++----
 Cargo.toml         | 15 ++++++++++++++-
 README.md          |  2 +-
 scripts/install.sh |  4 ++++
 scripts/script.sh  |  2 +-
 src/error.rs       |  6 ++++++
 src/github.rs      |  9 +++++++++
 src/lib.rs         | 15 ++++++++++++++-
 src/main.rs        |  6 +++++-
 src/templates.rs   |  8 +++++---
 src/travis.rs      | 40 ++++++++++++++++++++++++++++++++++++++++
 11 files changed, 103 insertions(+), 12 deletions(-)
```


