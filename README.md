
1. Get your tooling all set up (at least `rustup target add wasm32-unknown-unknown`)
1. Run ./release.sh

You can just serve this directory and load `./index.html` in a browser.

If you're dumb like me you can:

```
cargo install dsfs
./release.sh
dsfs
```

And point your browser to the URL printed on the console.

In the browser console, you'll see messages from `main.rs` but not from `task.rs`.

What up with that?
