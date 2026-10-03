---
"pacquet": patch
---

Fixed `pnpm install --frozen-lockfile` rejecting a fresh lockfile when an injected workspace dependency declares a peer dependency with the `catalog:` protocol [pnpm/pnpm#16557](https://github.com/pnpm/pnpm/issues/16557).
