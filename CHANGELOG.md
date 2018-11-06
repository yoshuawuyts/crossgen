## 2018-11-06, Version 0.6.0
### Commits
- [[`e3b011c249`](https://github.com/yoshuawuyts/crossgen/commit/e3b011c249fe056677d910cc6a2554106ac2b7d3)] (cargo-release) version 0.6.0 (Yoshua Wuyts)
- [[`d7ae39d7b2`](https://github.com/yoshuawuyts/crossgen/commit/d7ae39d7b28592e066611d5dd87cc6b10493e9d1)] Windows support (Travis CI) (#21) (Dan Reeves)
- [[`8532f8362e`](https://github.com/yoshuawuyts/crossgen/commit/8532f8362ed3d2e24a4ec0bb752958686ab01ff0)] Read Cargo.toml from target dir (#18) (Bruno Tavares)
- [[`566b811407`](https://github.com/yoshuawuyts/crossgen/commit/566b811407ef06d80a770e332b9080e7aa2c64e5)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 CHANGELOG.md             | 16 ++++++++++++++++
 Cargo.toml               |  2 +-
 src/main.rs              |  3 ++-
 templates/bin/install.sh | 16 ++++------------
 templates/bin/script.sh  | 20 ++++++++++++++------
 templates/bin/travis.yml |  3 +++
 templates/lib/install.sh | 16 ++++------------
 templates/lib/script.sh  | 20 ++++++++++++++------
 templates/lib/travis.yml |  3 +++
 9 files changed, 61 insertions(+), 38 deletions(-)
```


## 2018-10-11, Version 0.5.1
### Commits
- [[`6144374dd8`](https://github.com/yoshuawuyts/crossgen/commit/6144374dd8cad0bc61b807ea8a8570cab1601337)] (cargo-release) version 0.5.1 (Yoshua Wuyts)
- [[`1ff4ac19d4`](https://github.com/yoshuawuyts/crossgen/commit/1ff4ac19d4fe883ee6603fdfc5ace6c4808ac995)] fix openssl upgrade (#20) (Yoshua Wuyts)
- [[`c626698dab`](https://github.com/yoshuawuyts/crossgen/commit/c626698dab8e82e57da18b86033f6a45610bb700)] upgrade deps (Yoshua Wuyts)
- [[`48fa6fca63`](https://github.com/yoshuawuyts/crossgen/commit/48fa6fca63f5662807d7c0ae8995fa8099bda3c5)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 .travis.yml  |  4 ----
 CHANGELOG.md | 60 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
 Cargo.toml   | 46 ++++++++++++++++++++++------------------------
 3 files changed, 82 insertions(+), 28 deletions(-)
```


## 2018-09-19, Version 0.5.0
### Commits
- [[`c10f6f432b`](https://github.com/yoshuawuyts/crossgen/commit/c10f6f432b1ec86085fbde79ab70315317b1d1a2)] (cargo-release) version 0.5.0 (Yoshua Wuyts)
- [[`687967492c`](https://github.com/yoshuawuyts/crossgen/commit/687967492cbc2aecd287c73d33d66bca43bb5e9a)] Adding `lib` support to crossgen (#12) (Katharina)
- [[`0d4d893706`](https://github.com/yoshuawuyts/crossgen/commit/0d4d893706dbfd9fbe462f617aa6a8240f4a12e6)] Introduce 'name_from_dir' concept (#9) (Pascal Hertleif)
- [[`094c885926`](https://github.com/yoshuawuyts/crossgen/commit/094c8859261efa9a2e777afd7be1e68322bb1de7)] fix changelog (Yoshua Wuyts)
- [[`a4f8797f5f`](https://github.com/yoshuawuyts/crossgen/commit/a4f8797f5fdb21dad89cdd2cdb23b9775bc73a1e)] (cargo-release) version 0.4.0 (Yoshua Wuyts)
- [[`dd207e51a5`](https://github.com/yoshuawuyts/crossgen/commit/dd207e51a52a8339d653752a3a0a08b2e78faf25)] warning messages on wrong access (Yoshua Wuyts)
- [[`5f0b0ef036`](https://github.com/yoshuawuyts/crossgen/commit/5f0b0ef036f7a0d518bc3d3837a45a13a4f906eb)] re-test key encryption (Yoshua Wuyts)
- [[`f63d07d4eb`](https://github.com/yoshuawuyts/crossgen/commit/f63d07d4eb939baab3d962cf12123536582ade08)] (cargo-release) version 0.3.0 (Yoshua Wuyts)
- [[`f595065e75`](https://github.com/yoshuawuyts/crossgen/commit/f595065e75e421d1faf606cb5a29356cd1a29882)] fix github token creation (Yoshua Wuyts)
- [[`bfc40598b3`](https://github.com/yoshuawuyts/crossgen/commit/bfc40598b3f979e6c0511b1d28b9cb975f7e6c8c)] update changelog (Yoshua Wuyts)
- [[`5066d267d7`](https://github.com/yoshuawuyts/crossgen/commit/5066d267d70836d5652e9ee3b8eabc476ef05896)] (cargo-release) version 0.2.0 (Yoshua Wuyts)
- [[`edfaca59d0`](https://github.com/yoshuawuyts/crossgen/commit/edfaca59d08654e5536c5ba18834e564dc680277)] GitHub keysign (#6) (Yoshua Wuyts)
- [[`7d84a0201a`](https://github.com/yoshuawuyts/crossgen/commit/7d84a0201a77b325e09d2a70296fd5f75684cf7e)] Fix install command (#5) (Dan Reeves)
- [[`92602ed353`](https://github.com/yoshuawuyts/crossgen/commit/92602ed35331f238bf3b0caac1f454cf2925fc85)] (cargo-release) version 0.1.4 (Yoshua Wuyts)
- [[`5899b4fdb1`](https://github.com/yoshuawuyts/crossgen/commit/5899b4fdb1d2462911ce7d0cd20425b7a4c2d5b5)] (cargo-release) version 0.1.3 (Yoshua Wuyts)
- [[`d4d262a486`](https://github.com/yoshuawuyts/crossgen/commit/d4d262a486082b1a7d7519019ad8e9c95f9ab61d)] include release link (Yoshua Wuyts)
- [[`463d52fc83`](https://github.com/yoshuawuyts/crossgen/commit/463d52fc83fb5a42574a1cd2362fe15c057db273)] fix tests (Yoshua Wuyts)
- [[`423620b26e`](https://github.com/yoshuawuyts/crossgen/commit/423620b26e3dc53b3d9886551be070949ce6589b)]  mention trust (Yoshua Wuyts)
- [[`8363aa015d`](https://github.com/yoshuawuyts/crossgen/commit/8363aa015dde4e117a06e24b2770881459d80880)] (cargo-release) version 0.1.2 (Yoshua Wuyts)
- [[`44ad9ce188`](https://github.com/yoshuawuyts/crossgen/commit/44ad9ce188d091ddd4216cad21e723dab59fe5eb)] fix before_deploy (Yoshua Wuyts)
- [[`bb152c96fc`](https://github.com/yoshuawuyts/crossgen/commit/bb152c96fc59edfdf4642172985d8ed7c690124c)] (cargo-release) version 0.1.1 (Yoshua Wuyts)
- [[`65ecdd0ab0`](https://github.com/yoshuawuyts/crossgen/commit/65ecdd0ab095f2482d4220092defde9f243e81d8)] fix before_deploy (Yoshua Wuyts)
- [[`bf478c3441`](https://github.com/yoshuawuyts/crossgen/commit/bf478c3441a263d6a9b04bea9b6294059155cd2c)] update readme (Yoshua Wuyts)
- [[`b5779b7930`](https://github.com/yoshuawuyts/crossgen/commit/b5779b7930d3de779998ca5151089d68cf6aa89e)] feed to self (Yoshua Wuyts)
- [[`ea853e9469`](https://github.com/yoshuawuyts/crossgen/commit/ea853e9469f657740db07ef5cb33f09be2121f03)] fix warnings (Yoshua Wuyts)
- [[`2dfe5d1cdd`](https://github.com/yoshuawuyts/crossgen/commit/2dfe5d1cdd35e26423218d12aea38e8e40a36f97)] init (Yoshua Wuyts)
- [[`ef25131d57`](https://github.com/yoshuawuyts/crossgen/commit/ef25131d57555d6a490d6fed0eb15da6a198cdc1)] travis (Yoshua Wuyts)
- [[`0f5a0a18dc`](https://github.com/yoshuawuyts/crossgen/commit/0f5a0a18dca50c231e556d26840ca8367132d63d)] . (Yoshua Wuyts)

### Stats
```diff
 CHANGELOG.md                    | 18 +++++++++++-
 Cargo.toml                      |  2 +-
 src/cli.rs                      | 28 ++++++++++++------
 src/main.rs                     |  8 ++++-
 src/templates.rs                | 66 +++++++++++++++++++++++++++++++++++-------
 templates/appveyor.yml          |  0
 templates/before_deploy.ps1     | 23 +---------------
 templates/before_deploy.sh      | 32 +--------------------
 templates/bin/appveyor.yml      |  0
 templates/bin/before_deploy.ps1 | 23 +++++++++++++++-
 templates/bin/before_deploy.sh  | 32 ++++++++++++++++++++-
 templates/bin/install.sh        | 51 ++++++++++++++++++++++++++++++++-
 templates/bin/script.sh         | 23 +++++++++++++++-
 templates/bin/travis.yml        | 55 +++++++++++++++++++++++++++++++++++-
 templates/install.sh            | 51 +--------------------------------
 templates/lib/appveyor.yml      |  0
 templates/lib/before_deploy.ps1 | 23 +++++++++++++++-
 templates/lib/before_deploy.sh  | 32 ++++++++++++++++++++-
 templates/lib/install.sh        | 51 ++++++++++++++++++++++++++++++++-
 templates/lib/script.sh         | 23 +++++++++++++++-
 templates/lib/travis.yml        | 55 +++++++++++++++++++++++++++++++++++-
 templates/script.sh             | 23 +---------------
 templates/travis.yml            | 55 +-----------------------------------
 23 files changed, 469 insertions(+), 205 deletions(-)
```


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


