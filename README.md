# cargo-incver

A small utility for manipulating the project version in Cargo.toml

## CLI usage

Print full version number

```bash
cargo incver full get
```

Increment minor version

```bash
cargo incver minor inc
```

Set custom pre version

```bash
cargo incver pre set alpha
```

## Gitlab CI example

The example below shows an example of gitlab jobs that increment version numbers depending on the name of the merge request source branch

```yaml
increment-patch:
  stage: deploy
  image: simensgreen/incver
  rules:
    - if: $CI_MERGE_REQUEST_SOURCE_BRANCH_NAME =~ /^patch/
  script:
    - cargo incver patch inc

increment-minor:
  stage: deploy
  image: simensgreen/incver
  rules:
    - if: $CI_MERGE_REQUEST_SOURCE_BRANCH_NAME =~ /^minor/
  script:
    - cargo incver minor inc

```

Where "simensgreen/incver" image is just:

```Dockerfile
FROM rust
RUN cargo install cargo-incver
```
