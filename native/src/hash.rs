use neon::prelude::*;
use cryptonote_raw_crypto::{hash::Hash};
use std::fmt::Write;

pub fn get_fast_hash(mut cx: FunctionContext) -> JsResult<JsString> {
    let mut b: Handle<JsBuffer> = cx.argument(0)?;
    let data = cx.borrow(&mut b, |data| {
        let slice = data.as_slice::<u8>();
        slice
    });
    let hash = Hash::fast(data);
    let mut s = String::new();
    for &byte in hash.iter() {
        write!(&mut s, "{:02x}", byte).expect("Unable to write");
    }
    Ok(cx.string(s))
}

pub fn get_slow_hash(mut cx: FunctionContext) -> JsResult<JsString> {
    let mut b: Handle<JsBuffer> = cx.argument(0)?;
    let variant = cx.argument::<JsNumber>(1)?.value() as usize;
    let data = cx.borrow(&mut b, |data| {
        let slice = data.as_slice::<u8>();
        slice
    });
    let hash = Hash::slow_with_variant(data, variant);
    let mut s = String::new();
    for &byte in hash.iter() {
        write!(&mut s, "{:02x}", byte).expect("Unable to write");
    }
    Ok(cx.string(s))
}
