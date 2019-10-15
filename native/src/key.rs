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

pub fn secret_key_to_public_key(mut cx: FunctionContext) -> JsResult<JsArrayBuffer> {
    let secret_key = get_hash(&mut cx, 0);
    let mut public_key: [u8;32] = [0;32];
    let result = Key::secret_to_public(&secret_key, &mut public_key);
    let mut buffer = JsArrayBuffer::new(&mut cx, 32)?;
    if result {
        cx.borrow_mut(&mut buffer, |data| {
            let slice = data.as_mut_slice();
            for i in 0..32 {
                slice[i] = public_key[i];
            }
        });
    }
    Ok(buffer)
}


pub fn generate_key_derivation(mut cx: FunctionContext) -> JsResult<JsArrayBuffer> {
    let public_key = get_hash(&mut cx, 0);
    let secret_key = get_hash(&mut cx, 1);
    let derived = Key::generate_key_derivation(&public_key, &secret_key);

    let mut buffer = JsArrayBuffer::new(&mut cx, 32)?;
    cx.borrow_mut(&mut buffer, |data| {
        let slice = data.as_mut_slice();
        for i in 0..32 {
            slice[i] = derived[i];
        }
    });
    Ok(buffer)
}

pub fn derive_public_key(mut cx: FunctionContext) -> JsResult<JsArrayBuffer> {
    let derivation = get_hash(&mut cx, 0);
    let public_key = get_hash(&mut cx, 1);
    let out_index = cx.argument::<JsNumber>(2)?.value();
    let derived = Key::derive_public_key(&derivation, out_index as u64, &public_key);

    let mut buffer = JsArrayBuffer::new(&mut cx, 32)?;
    cx.borrow_mut(&mut buffer, |data| {
        let slice = data.as_mut_slice();
        for i in 0..32 {
            slice[i] = derived[i];
        }
    });
    Ok(buffer)
}


pub fn derive_secret_key(mut cx: FunctionContext) -> JsResult<JsArrayBuffer> {
    let derivation = get_hash(&mut cx, 0);
    let secret_key = get_hash(&mut cx, 1);
    let out_index = cx.argument::<JsNumber>(2)?.value();
    let derived = Key::derive_secret_key(&derivation, out_index as u64, &secret_key);

    let mut buffer = JsArrayBuffer::new(&mut cx, 32)?;
    cx.borrow_mut(&mut buffer, |data| {
        let slice = data.as_mut_slice();
        for i in 0..32 {
            slice[i] = derived[i];
        }
    });
    Ok(buffer)
}


pub fn underive_public_key(mut cx: FunctionContext) -> JsResult<JsArrayBuffer> {
    let derivation = get_hash(&mut cx, 0);
    let public_key = get_hash(&mut cx, 1);
    let out_index = cx.argument::<JsNumber>(2)?.value();
    let derived = Key::underive_public_key(&derivation, out_index as u64, &public_key);

    let mut buffer = JsArrayBuffer::new(&mut cx, 32)?;
    cx.borrow_mut(&mut buffer, |data| {
        let slice = data.as_mut_slice();
        for i in 0..32 {
            slice[i] = derived[i];
        }
    });
    Ok(buffer)
}

pub fn generate_key_image(mut cx: FunctionContext) -> JsResult<JsArrayBuffer> {
    let public_key = get_hash(&mut cx, 0);
    let secret_key = get_hash(&mut cx, 1);
    let image = Key::generate_key_image(&public_key, &secret_key);

    let mut buffer = JsArrayBuffer::new(&mut cx, 32)?;
    cx.borrow_mut(&mut buffer, |data| {
        let slice = data.as_mut_slice();
        for i in 0..32 {
            slice[i] = image[i];
        }
    });
    Ok(buffer)
}

