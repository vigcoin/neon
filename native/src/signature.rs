use cryptonote_raw_crypto::{key::Key, ring::Ring};
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


pub fn check_ring_signature(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let prefix_hash = get_hash(&mut cx, 0);
    let image = get_hash(&mut cx, 1);
    let ba: Handle<JsArray> = cx.argument::<JsArray>(2).expect("Fail to parse Array!");
    let vec: Vec<Handle<JsValue>> = ba.to_vec(&mut cx).expect("Fail to parse vec!");
    let pubs = to_buffer_array(&mut cx, &vec);
    let pubs_count = cx.argument::<JsNumber>(0)?.value();
    let signatures = get_buffer(&mut cx, 4);
    let key = Ring::check_signature(&prefix_hash, &image, &pubs, pubs_count as usize, &signatures);
    Ok(cx.boolean(key))
}


pub fn generate_ring_signature(mut cx: FunctionContext) -> JsResult<JsArrayBuffer> {
    let prefix_hash = get_hash(&mut cx, 0);
    let image = get_hash(&mut cx, 1);
    let ba: Handle<JsArray> = cx.argument::<JsArray>(2).expect("Fail to parse Array!");
    let vec: Vec<Handle<JsValue>> = ba.to_vec(&mut cx).expect("Fail to parse vec!");
    let pubs = to_buffer_array(&mut cx, &vec);
    let pubs_count = cx.argument::<JsNumber>(3)?.value() as usize;

    let secret_key = get_hash(&mut cx, 4);
    let secret_key_index = cx.argument::<JsNumber>(5)?.value() as usize;
    let signature = Ring::generate_signature(
        &prefix_hash,
        &image,
        &pubs,
        pubs_count,
        &secret_key,
        secret_key_index,
    );

    let len = pubs_count * 64;
    let mut buffer = JsArrayBuffer::new(&mut cx, len as u32)?;
    cx.borrow_mut(&mut buffer, |data| {
      let slice = data.as_mut_slice();
      for i in 0..len {
        slice[i] = signature[i];
      }
    });
    Ok(buffer)
}


