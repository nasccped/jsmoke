JSmoke architecture
===================

Jsmoke Architecture related stuff.

> [!IMPORTANT]
>
> All images here was made using [`draw.io`](https://www.drawio.com/)
> and/or [`Camunda`](https://camunda.com/) free features.

## Proposal

Inspired by the [`cargo`](https://github.com/rust-lang/cargo) app,
jsmoke is intended to generate, compile and run java project through
cli inputs, like [`maven`](https://maven.apache.org/) and
[`gradle`](https://gradle.org/), but avoiding unnecessary complexity.

### JSmoke pros

- avoid long file-tree boilerplate code:
```
project-root/
+- .mvn/
|  +- ...
+- src/
|  +- main/
|     +- java/
|        +- com/
|           +- ...
+- mvnw
+- mvnw.cmd
+- ...
```
- faster applications development/compiling:
    - jsmoke is built under [`rust lang`](https://www.rust-lang.org/)
      (compiled) instead of java/kotlin
    - no thorough check, just your app needs
    - source file tree smaller, so no deep source navigation
- simpler usage: [`cargo`](https://github.com/rust-lang/cargo)
  inspired cli design

### JSmoke cons

- no maven central importing: since jsmoke doesn't have maven's
  dependency feature, it doesn't resolve dependency artifacts
- no tests: java related projects are tested through test frameworks
  ([`JUnit`](https://junit.org/),
  [`Mockito`](https://site.mockito.org/), _a lot more_) _(read bullet
  above)_
- no corporate features: a lot of corporate stuff is done through
  maven
- no env testing: things like `java`/`javac` version and `JAVA_HOME`
   aren't properly checked before programs start

### Overall result

😁 You should use this program if:

- you're a programming student/enjoyer
- you want to create and use your project in a fast way
- your project is guaranteed simple
- you want to give a try to _open source indie projects_

☹️ You should **NOT** use this program if:

- you're building a professional related application
- you want to use in a commercial product
- your project have external dependencies
- you're having high expectations

> [!NOTE]
>
> Don't get me wrong. This isn't a _"next gen product"_. It's manager
> alternative for curious people (like me `^^`) to understand
> project, product and value development.
