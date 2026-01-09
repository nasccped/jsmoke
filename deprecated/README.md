jsmoke libraries situation
==========================

jsmoke was built under the monorepo philosophy since cargo's
workspace allows it. But unfortunately, it turns a lot harder
to mantain, fix and update the entire project (as single-developer).

With this in mind, the jsmoke app no longer supports it's personal
libraries (`jsmoke_cli`, `jsmoke_application`, `jsmoke_macros`,
`jsmoke_utils`) and now, follows a monolith app-design!

This subdir just holds the latest version of each library to allow
easy tracking over versions and/or future changes.

## Licensing

Even with no LICENSE file within each library, they're still under
the MIT license as specified on each `Cargo.toml`. The raw file can
be found at [../LICENSE](../LICENSE)!
