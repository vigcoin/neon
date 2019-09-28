

use cryptonote_raw_crypto::{scalar::EllipticCurveScalar};
use neon::prelude::*;

use util::*;

  extern "C" {
    fn setup_random(value: i32);
  }

pub fn is_scalar(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let scalar = get_hash(&mut cx, 0);
    let is_scalar = EllipticCurveScalar::check(&scalar);
    Ok(cx.boolean(is_scalar))

}

pub fn init_random(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let value = cx.argument::<JsNumber>(0)?.value();

    unsafe {
      setup_random(value as i32);
    }
    Ok(cx.undefined())
}

pub fn random_scalar(mut cx: FunctionContext) -> JsResult<JsArrayBuffer> {
    let mut ec_scalar: [u8; 32] = [0; 32];
    EllipticCurveScalar::random(&mut ec_scalar);
    let mut buffer = JsArrayBuffer::new(&mut cx, 32)?;
    cx.borrow_mut(&mut buffer, |data| {
      let slice = data.as_mut_slice();
      for i in 0..32 {
        slice[i] = ec_scalar[i];
      }
    });
    Ok(buffer)
}

pub fn hash_to_scalar(mut cx: FunctionContext) -> JsResult<JsArrayBuffer> {
    let scalar = get_buffer(&mut cx, 0);
    let hash = EllipticCurveScalar::to_hash(&scalar);
    let mut buffer = JsArrayBuffer::new(&mut cx, 32)?;
    cx.borrow_mut(&mut buffer, |data| {
      let slice = data.as_mut_slice();
      for i in 0..32 {
        slice[i] = hash[i];
      }
    });
    Ok(buffer)
}