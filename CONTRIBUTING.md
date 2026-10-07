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

note that this project uses hard tabs rather than 4 spaced indents as a design choice,
if you made your PR with four spaced indents, please run `cargo fmt` after checking `cargo fmt --check`.

you should format your code using cargo, example below:
```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```
you can run these commands manually, however i reccomend using the makefile to automate that process.
early versions of CRUSH that aren't ready yet will fail at the cargo clippy step, to bypass this,
you will need to call the parts of the makefile individually or manually run the commands in the makefile.
in release versions of CRUSH, the CI standards will apply fully, but for now, in the early phases of the project,
the CI standards are still optional.
by default, the makefile does NOT modify code, unless you specifically call the format command, do note that
cargo fmt can have flaws and those flaws could in theory change behavior.

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

the following prefixes are reserved for the project owner:
- style: change project style (i.e. hard tabs > four spaced indent)
- CI: change CI processes

while these prefixes are reserved for the project owner, regular maintainers and contributors, or even
just regular users can still reccomend those changes.
if you have an issue with how CI/style is handled currently, please open an issue or a discussion rather than a PR,
unless that change has already been discussed.
