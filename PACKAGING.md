# CRUSH Packaging standards

CRUSH is a shell, therefore packaging is an integral part of distribution of the project.
as such, CRUSH has some packaging standards.

## Changes
changes to the shell in packaging should be discussed with maintainers and the project owner.
of course, i can't stop you from packaging a slightly modified version, however you must cleanly
label your package as modified from upstream.

## Platforms
as of now, there is no packaging or adoption of CRUSH.
plans for packaging are the following:
- cargo crate & AUR first to get first adoption by more advanced users
- eventual adoption by a trusted maintainer to get into arch's extra
packaging for distributions other than arch is not yet planned, you are welcome to propose packaging
to maintainers, or if you have the privileges to, packaging it yourself for said repos.

## Issues & Bugs
please only point users to upstream bug reporting if the issue actually comes from upstream.
if you are shipping outdated versions of CRUSH (for example on debian), you should backport bug fixes
to your packaged version if it makes sense and is possible.
security issue fixes should always be backported.
as a packager for CRUSH, you are responsible for your package, if it does not backport bug fixes or
stay up to date with versioning, thats on you.
