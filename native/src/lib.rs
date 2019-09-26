#[macro_use]
extern crate neon;
extern crate cryptonote_wallet;
extern crate cryptonote_raw_crypto;
extern crate hex;

use cryptonote_wallet::{Wallet};
use cryptonote_raw_crypto::{hash::Hash, key::Key, ring::Ring};
use neon::prelude::*;
use std::fmt::Write;

#[no_mangle]
pub extern fn __cxa_pure_virtual() {
    loop{};
}

fn get_buffer(cx: &mut neon::context::CallContext<'_, neon::types::JsObject>, idx: i32) -> Vec<u8>{
    let mut b: Handle<JsBuffer> = cx.argument(idx).expect("Fail to get argument!");
    let data = cx.borrow(&mut b, |data| {
      let slice = data.as_slice::<u8>();
      slice
    });
    let mut buffer: Vec<u8> = vec![];
    for i in 0..data.len() {
        buffer.push(data[i]);
    }
    buffer
}

fn get_hash(cx: &mut neon::context::CallContext<'_, neon::types::JsObject>, idx: i32) -> [u8; 32]{
    let data = get_buffer(cx, idx);
    let mut fixed_data : [u8; 32] = [0; 32];
    for i in 0..32 {
        fixed_data[i] = data[i];
    }
    fixed_data
}

fn get_fast_hash(mut cx: FunctionContext) -> JsResult<JsString> {
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

fn is_public_key(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let key = get_hash(&mut cx, 0);
    let is_key = Key::check_public_key(&key);
    Ok(cx.boolean(is_key))
}
/*
pub fn check_signature(
    prefix_hash: &[u8; 32],
    image: &[u8; 32],
    pubs: &Vec<[u8; 32]>,
    pubs_count: usize,
    signatures: &Vec<u8>
) -> bool
*/
// fn check_ring_signature(mut cx: FunctionContext) -> JsResult<JsBoolean> {
//     let prefix_hash = get_hash(&mut cx, 0);
//     let image = get_hash(&mut cx, 1);
//     let key = Ring::check_signature(&prefix_hash, &image,);
//     Ok(cx.boolean(key))
// }

declare_types! {
    /// JS class wrapping Employee records.
    pub class JsWallet for Wallet {
        init(mut cx) {
            let filename = cx.argument::<JsString>(0)?.value();
            let password = cx.argument::<JsString>(1)?.value();

            let mut wallet = Wallet::new();
            if filename.len() > 0 {
              wallet.load(filename, password);
            }
            Ok(wallet)
        }

        method create(mut cx) {
            let prefix = cx.argument::<JsNumber>(0)?.value() as u64;
            let js_object = JsObject::new(&mut cx);
            let wallet = Wallet::create();
            let address = wallet.to_address(prefix);
            let keys = (wallet.spend_keys.0, wallet.view_keys.0);
            let spend = hex::encode(keys.0);
            let view = hex::encode(keys.1);
            let spend_str = cx.string(spend);
            let view_str = cx.string(view);
            let address_str = cx.string(address.get());
            js_object.set(&mut cx, "spend", spend_str)?;
            js_object.set(&mut cx, "view", view_str)?;
            js_object.set(&mut cx, "address", address_str)?;
            Ok(js_object.upcast())
        }

        method setPrivateKeys(mut cx) {
            let spend = cx.argument::<JsString>(0)?.value();
            let view = cx.argument::<JsString>(1)?.value();
            let mut this = cx.this();
            {
            let guard = cx.lock();
            let mut wallet = this.borrow_mut(& guard);
            wallet.update_secret_keys(spend, view);
            };
            Ok(cx.undefined().upcast())
        }

        method getPrivateKeys(mut cx) {
            let js_object = JsObject::new(&mut cx);
            let this = cx.this();
            let keys =
            {
            let guard = cx.lock();
            let wallet = this.borrow(& guard);
            (wallet.spend_keys.0, wallet.view_keys.0)
            };
            let spend = hex::encode(keys.0);
            let view = hex::encode(keys.1);
            let spend_str = cx.string(spend);
            let view_str = cx.string(view);
            js_object.set(&mut cx, "spend", spend_str)?;
            js_object.set(&mut cx, "view", view_str)?;
            Ok(js_object.upcast())
        }

        method save(mut cx) {
            let this = cx.this();
            let filename = cx.argument::<JsString>(0)?.value();
            let password = cx.argument::<JsString>(1)?.value();
            {
            let guard = cx.lock();
            let wallet = this.borrow(&guard);
            wallet.save(filename, password);
            };
            Ok(cx.undefined().upcast())
        }

        method toAddress(mut cx) {
            let this = cx.this();
            let prefix = cx.argument::<JsNumber>(0)?.value() as u64;
            let address = {
            let guard = cx.lock();
            let wallet = this.borrow(&guard);
            let address = wallet.to_address(prefix);
            address
            };
            Ok(cx.string(address.get()).upcast())
        }
    }
}

register_module!(mut cx, {
    cx.export_class::<JsWallet>("Wallet")?;
    cx.export_function("getFastHash", get_fast_hash)?;
    cx.export_function("isPublicKey", is_public_key)?;
    Ok(())
});
