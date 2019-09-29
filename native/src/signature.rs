use cryptonote_raw_crypto::{key::Key};
use neon::prelude::*;
use util::*;


pub fn generate_signature(mut cx: FunctionContext) -> JsResult<JsArrayBuffer> {
    let prefix_hash = get_hash(&mut cx, 0);
    let public_key = get_hash(&mut cx, 1);
    let secret_key = get_hash(&mut cx, 2);
    let signature = Key::generate_signature(
      &prefix_hash,
      &public_key,
      &secret_key
    );
    let mut buffer = JsArrayBuffer::new(&mut cx, 64)?;
    cx.borrow_mut(&mut buffer, |data| {
      let slice = data.as_mut_slice();
      for i in 0..64 {
        slice[i] = signature[i];
      }
    });
    Ok(buffer)
}

pub fn check_signature(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let prefix_hash = get_hash(&mut cx, 0);
    let public_key = get_hash(&mut cx, 1);
    let signature = get_signature(&mut cx, 2);
    let check = Key::check_signature(
      &prefix_hash,
      &public_key,
      &signature
    );
    Ok(cx.boolean(check))
}


