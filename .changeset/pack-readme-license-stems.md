---
"pacquet": patch
---

`pnpm pack` and `pnpm publish` always include only the root files named `README` or `LICENSE`, with or without an extension. Files that just start with those names, such as `README_INTERNAL.md` or `LICENSE-MIT`, now follow the `files` field and `.npmignore` like any other file [#16753](https://github.com/pnpm/pnpm/issues/16753).
