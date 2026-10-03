# Contributing to CRUSH

as CRUSH is a shell, there has to be strict contribution standards.
the standards are listed below:

## AI Usage
AI Usage is allowed, but has several nuances.
for further information, check README.md
as a contributor, you must understand your code.


## PRs
a PR needs a clear purpose.
you should:
- explain what it changes
- explain why it's needed
- tell us what problem it solves
- how it was tested (distro, libc, compiler, rust/cargo versions)

PRs should also address only a few features at a time, they should not be monolithic rewrites.
if you're for example planning to add a feature, fix a bug and add support for an OS, seperate that
into 3 PRs, rather than 1 huge one.

you should format your code using cargo, example below:
```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```
note that these will eventually be added to an automation script, to automate checks of
proper formatting, warnings in the code and testing if the project passes test
you must mention in your PR if theres any formatting issues, warnings or tests that weren't passed
that you could not fix, and they must be fixed before a merge.

PRs should not bring in unnecessary dependencies, only bring in dependencies if they make sense.
also avoid rewriting systems just because you prefer a different philosophy.
major rewrites must be discussed before being merged, and you must get a green light from a majority
of maintainers, or the project owner before it gets merged.
if your major rewrite does not get a green light, you should fork the project instead.

you should also test if basic behavior changes, and document it in your PR.
changes to behavior must be documented and intentional.
you should also update the .md files if something changes thats mentioned in them.

avoid spamming commits,or making unnecessary commits. the commit history should remain somewhat clean.
also prefix your commit with what kind of change it is.
for example these:
- fix: bug name/issue reference
- test: add tests for x
- refactor: optimize x
- docs: make documentation clearer in x
- add: add x because y
- extend: add x to feature z because y
- security: fix vulnerability
