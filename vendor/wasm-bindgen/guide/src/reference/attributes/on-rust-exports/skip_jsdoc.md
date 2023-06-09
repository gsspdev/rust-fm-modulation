# `skip_jsdoc`

When attached to a function or a method, prevents `wasm-bindgen` from auto-generating JSDoc-style doc comments.
By default, `wasm-bindgen` adds `@param` and `@returns` annotations to doc comments in the generated
JS files. A `skip_jsdoc` annotation prevents this, allowing you to supply your own doc comments.

The following rust uses `skip_jsdoc` to omit one of the auto-generated doc comments.

```rust
use wasm_bindgen::prelude::*;

/// Autogenerated docs.
#[wasm_bindgen]
pub fn foo(arg: u32) -> u32 { arg + 1 }

/// Manually written docs.
///
/// @param {number} arg - A descriptive description.
/// @returns {number} Something a bit bigger.
#[wasm_bindgen(skip_jsdoc)]
pub fn bar(arg: u32) -> u32 { arg + 2 }
```

The `wasm-bindgen`-generated JS interface of the above code will look something like this:

```js
/**
* Autogenerated docs.
*
* @param {number} arg
* @returns {number}
*/
export function foo(arg) { /* ... */ }

/**
* Manually written docs.
*
* @param {number} arg - A descriptive description.
* @returns {number} Something a bit bigger.
*/
export function bar(arg) { /* ... */ }
```
