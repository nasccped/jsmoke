# JSmoke documentation

A more deep understanding about JSmoke, it's features and philosophy.

## Table of contents

- [A little about](#a-little-about)
  - [Naming](#naming)
  - [Why does JSmoke exists?](#why-does-jsmoke-exists)

## A little about

JSmoke is a java project manager intended to be simple and run fast
enough that you forget you're using a manager.

### Naming

The _"JSmoke"_ comes from _"java smoke"_. A cup of coffee blazingly
hot (written in Rust, btw).

### Why does JSmoke exists?

When writing java programs, I noticed that a [Makefile] was kinda
boring to rewrite everytime a new project was created (still better
than passing file paths by the cmdline). Even using [maven] or
[gradle] looks bad. It's a too-heavy tooling for my simple todo-list.

So I though:

> Imagine if the people could create and run it's projects just by
> using the terminal in the simplest way...
>
> No overloaded and fancy IDEs. No heavy-tooling overhead. Just you,
> the code and the final result + useful security/workflow features
> that won't take more than millisecs to run.

Then, JSmoke was born.

[maven]: https://maven.apache.org/
[gradle]: https://gradle.org/
[Makefile]: https://www.gnu.org/software/make/manual/make.html
