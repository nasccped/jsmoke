# JSmoke documentation

A more deep understanding about JSmoke, it's features and philosophy.

## Table of contents

- [A little about](#a-little-about)
  - [Naming](#naming)
  - [Why does JSmoke exists?](#why-does-jsmoke-exists)
- [Kojamp, its predecessor](#kojamp-its-predecessor)
- [Features](#features)
- [License](#license)

## A little about

JSmoke is a java project manager intended to be simple and run fast
enough that you forget you're using a manager.

### Naming

The _"JSmoke"_ comes from _"java smoke"_. A cup of coffee blazingly
hot (written in Rust, btw).

### Why does JSmoke exists?

When writing java programs, I noticed that a [Makefile] was kinda
boring to rewrite every time a new project was created (still better
than passing file paths by the cmdline). Even using [maven] or
[gradle] looks bad. It's a too-heavy tooling for my simple todo-list.

So I though:

> Imagine if the people could create and run it's projects just by
> using the terminal in the simplest way...
>
> No overloaded and fancy IDEs. No heavy-tooling overhead. Just you,
> the code and the final result + useful security/workflow features
> that won't take more than millisecs[^millisecs] to run.

Then, JSmoke was born.

## Kojamp, its predecessor

[Kojamp] was the first version of this software but was discontinued.
It was intended to offer support for both `java` and `kotlin`
applications.

The confuse java support turned impossible to implement `kotlin`
features. Also, due to aesthetic code principles and implementing
I've decided to redo the program from scratch with a new look.

> [!NOTE]
>
> Even discontinued, [Kojamp] still works _(not the way I want, but
> it does the job 😁)_.

## Features

Since JSmoke is a manager it provides similar features already
provided in other managers, such as [maven] or [gradle].

Here's a small check list to stay tuned with the project progress:

- [ ] 🆕 create new projects
- [ ] 🏗️ build _(compile)_ projects
- [ ] 🏎️ run project _(bytecode)_
- [ ] 🧼 clear generated files _(bytecode)_
- [ ] 📰 `stderr`[^stderr] report logging

## License

This project is under the [MIT License]. You can read the License
file by [clicking here](https://github.com/nasccped/jsmoke/blob/main/LICENSE)!

[maven]: https://maven.apache.org/
[gradle]: https://gradle.org/
[Makefile]: https://www.gnu.org/software/make/manual/make.html
[Kojamp]: https://github.com/nasccped/kojamp
[MIT License]: https://opensource.org/license/mit

[^millisecs]: `millisecs` stands for milliseconds, one thousandth
  (`0.001` or <code>10<sup>−3</sup></code> or `1/1000`) of a second
  (or `1000` microseconds).
[^stderr]: `stderr`, or **Standard Error**, is one of three standard
  data streams in Unix-like operating systems (the others being stdin
  for input and stdout for output). It is a dedicated channel for
  programs to output error messages, warnings, and diagnostic
  information.
