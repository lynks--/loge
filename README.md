loge
====

**loge - simple, fast terminal logging for rust**

_Overview_

Just want simple terminal logging macros? Feel like [slog](https://docs.rs/slog) is overkill for your project and
don't feel good about the marginal runtime costs of [log](https://docs.rs/log)? loge might be the doge for you.

_Features_

- No run-time verbosity level checking, zero-cost macros in the disabled case.
- Three simple levels: `debug!` (opt in), `info!` (opt out), and `warn!` (always on).

_Usage_

```rust
#[macro_use] extern crate loge;

debug!("about to do the thing..."); // only printed if you enable the loge-debug feature

if let Ok(done) = do_the_thing() {
	info!("thing done:  {:?}", done); // printed unless you enable the loge-no-info feature
}
else {
	warn!("failed to do the thing!"); // always printed
}
```
