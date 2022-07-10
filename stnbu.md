# Notes and watnot

* Does a general "Rust Metamask" (or eip1193) library exist? Bevy's great, but
If there's an unfilled niche... Maybemaybenot. I haven't really investigated.
* Some stuff defined in `metamask.rs` could be generalized:
  * Should the stuff in `AppData` be something different that's easily usable
  as a `Component` or `Resource`? We might want every user to have their own
  wallet, for example.
  * Why missing `mut` from `metamask_ch: ResMut`. Need it be mutable?
  * The signing stuff needs much expansion. Seems like: Signing something should
  Cause an event or similar...something actionable where we can get to the
  signatures.
* I guess maybe...
  * `metamask.rs` -> `lib.rs`
  * `main.rs` -> `examples/foo.rs`
* `cargo run --example sign` runs but dies. Not going to look into it because:
  this is a WASM crate. Is `wasm-server-runner` something we could seamlessly
  use in "examples"? Then `cargo run --example sign` could just print a URL to
  point your browser at.