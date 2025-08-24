jsmoke's about page
===================

Here, we'll talk about the jsmoke extra info.

## Table of contents

- [What is jsmoke](#what-is-jsmoke)
- [Kojamp, its ancestor](#kojamp-its-ancestor)
- [jsmoke features](#jsmoke-features)
- [jsmoke pros & cons](#jsmoke-pros--cons)
    - [jsmoke advantages](#jsmoke-advantages)
    - [jsmoke disadvantages](#jsmoke-disadvantages)
    - [Overall result](#overall-result)
- [License](#license)

## What is jsmoke?

**jsmoke** is a project manager that targets small java applications.
It works like [maven] and [gradle], but avoiding the _unnecessary_
boilerplate code and dependency resolving.

[maven]: https://maven.apache.org/
[gradle]: https://gradle.org/

## Kojamp, its ancestor

The **jsmoke** app is the **kojamp** successor. At first, kojamp was
intended to target Java and Kotlin applications, but a lot of
redesigns became necessary. You can find more info at
[kojamp's official repository].

[kojamp's official repository]: https://github.com/nasccped/kojamp

## jsmoke features

This is a simple **todo** list for the app's features:

> [!IMPORTANT]
>
> The stuff bellow is intended to be added to the app's core
> functionality. It can be changed for _fix_/_design_ purposes.
>
> Consider:
> - [x] <- as done
> - [ ] <- on the features queue

- [ ] create new projects
    - [ ] using `new` subcommand
    - [ ] using `init` subcommand
- [ ] compile jsmoke based projects
- [ ] run jsmoke based projects
- [ ] clean jsmoke based projects bytecode
- [ ] error reporting
    - [ ] when compiling
    - [ ] cli incorrect usage
- [ ] error storing (like log) when compilation fails
- [ ] version control system handling (like [cargo])
- [ ] **env config** handling
- [ ] _(More stuff may be added here later...)_

[cargo]: https://github.com/rust-lang/cargo

## jsmoke pros & cons

What differs jsmoke from other project managers?

### jsmoke advantages

- avoid deep file-tree: just `src` dir and `env` config files
- faster development:
  - you don't have to navigate through nested paths
  - compilation is managed with rust code (compiled)
  - no thorough checks, just your app needs
- simple usage: [cargo] inspired application

[cargo]: https://github.com/rust-lang/cargo

### jsmoke disadvantages

- no dependency support: the app doesn't operate under maven central
- no tests: test frameworks (mostly [JUnit] and [Mockito]) are
  dependency related fields _(read bullet above)_
- no corporate features: a lot of corporate stuff is done through
  maven
- no env testing: things like `java`/`javac` version and `JAVA_HOME`
   aren't properly checked before programs start

[JUnit]: https://junit.org/
[Mockito]: https://site.mockito.org/

### Overall result

😁 You should use this program if:

- you're a programming student/enjoyer
- you want to create and use your project in a fast way
- your project is guaranteed simple
- you want to give a try to _open source indie projects_

☹️ You should **NOT** use this program if:

- you're building a professional related application
- you want to use in a commercial product
- your project may have external dependencies
- you're having high expectations

> [!NOTE]
>
> Don't get me wrong. This isn't a _"next gen product"_. It's manager
> alternative for curious people (like me ^^) to understand project,
> product and value development.

## License

This project is under the [MIT license]. You can access it by clicking
[this link](https://github.com/nasccped/jsmoke/blob/main/LICENSE)!

[MIT license]: https://opensource.org/license/mit
