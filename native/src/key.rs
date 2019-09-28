use cryptonote_raw_crypto::{key::Key};
use neon::prelude::*;
use util::*;


pub fn generate_keys(mut cx: FunctionContext) -> JsResult<JsArrayBuffer> {
    let mut generated_public_key: [u8; 32] = [0; 32];
    let mut generated_private_key: [u8; 32] = [0; 32];
    Key::generate_key_pair(&mut generated_public_key, &mut generated_private_key);

    let mut buffer = JsArrayBuffer::new(&mut cx, 64)?;
    cx.borrow_mut(&mut buffer, |data| {
      let slice = data.as_mut_slice();
      for i in 0..32 {
        slice[i] = generated_public_key[i];
      }
      for i in 32..64 {
        slice[i] = generated_private_key[i - 32];
      }
    });
    Ok(buffer)
}


pub fn check_key(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let hash = get_hash(&mut cx, 0);
    Ok(cx.boolean(Key::check_public_key(&hash)))
}