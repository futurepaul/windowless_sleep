## windowless sleep

The web-sys crate uses the `window` object to access the global setTimeout function, which doesn't work in contexts like web workers, WASI, node.js, or, most importantly, bun.

This is a version of sleep that just uses whatever global setTimeout function happens to exist. No idea if this will help with WASI but it works in web workers and bun.

