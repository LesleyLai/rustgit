# Rust Git

Yet another Git clone. The purpose of this project is to help me understand git internals.

The objective is to develop something that can seamlessly substitute normal Git for day-to-day operations.

## Commands Implemented

**Porcelain Commands**

| Command | Note & Limitations              |
|---------|---------------------------------|
| `init`  | No support for reinitialization |

**Plumbing Commands**

| Command       | Note & Limitations                                                                |
|---------------|-----------------------------------------------------------------------------------|
| `cat-file`    | currently only supports `-p`                                                      |
| `hash-object` |                                                                                   |
| `ls-tree`     | currently only supports `--name-only`                                             |
| `write-tree`  | currently just write the whole folder without consider what's in the stage buffer |

## References

Here are some references I used during this project

- [Git book](https://git-scm.com/book/en/v2)
- [Codecrafters Build your own Git challenge](https://app.codecrafters.io/courses/git)
- [Building Git](https://shop.jcoglan.com/building-git/)
