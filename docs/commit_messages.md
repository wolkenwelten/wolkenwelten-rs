# How to write commit messages for WolkenWelten
This shall serve as a reminder mostly to myself about how to properly write
commit messages and make sure that a useful changelog can be generated automatically.

## Start with a keyword to add a changelog entry
Only commits that start with a keyword get added to the changelog,
this is to reduce noise since most users don't care that I added some
comments or updated some minor dependency. The keyword should be one of
the following:

- Break: Breaking changes, requiring action from devs
- Fix: This is for bugfixes
- New: Added a new feature
- Game: When something gameplay related changed
- Linux: Linux specific change/fix
- Mac: Linux specific change/fix
- Win: Windows specific change/fix

## Add a longer explanataion after the summary
This should be included in the changelog as well so that those interested can read the details without having to hunt for the exact commit.