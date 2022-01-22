This folder contains some tools that I use for this blog.

- splitter: Split `DEVLOG.md` into multiple individual posts, it was made to migrate this repository into the new structure.
- composer: The tool to combine the individual posts into a single `DEVLOG.md`, should be run everytime a new post is made.

There will be a pre-commit git hook that run the `composer` command before every commit, to rebuild the `DEVLOG.md` file.
