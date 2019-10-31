#[macro_use]
extern crate neon;
extern crate cryptonote_wallet;
extern crate cryptonote_raw_crypto;
extern crate hex;

mod util;
mod scalar;
mod key;
mod signature;
mod hash;
mod difficulty;

use scalar::{*};
use key::{*};
use signature::{*};
use hash::{*};
use difficulty::{*};

use cryptonote_wallet::{Wallet};
use neon::prelude::*;

#[no_mangle]
pub extern fn __cxa_pure_virtual() {
    loop{};
}

declare_types! {
    /// JS class wrapping Wallet functions
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
    cx.export_class::<JsDifficulty>("Difficulty")?;
    cx.export_function("getFastHash", get_fast_hash)?;
    cx.export_function("getSlowHash", get_slow_hash)?;
    cx.export_function("checkHash", check_with_difficulty)?;
    cx.export_function("isScalar", is_scalar)?;
    cx.export_function("setupRandom", init_random)?;
    cx.export_function("randomScalar", random_scalar)?;
    cx.export_function("hashToScalar", hash_to_scalar)?;
    cx.export_function("generateKeys", generate_keys)?;
    cx.export_function("checkKey", check_key)?;
    cx.export_function("secretKeyToPublicKey", secret_key_to_public_key)?;
    cx.export_function("generateKeyDerivation", generate_key_derivation)?;
    cx.export_function("derivePublicKey", derive_public_key)?;
    cx.export_function("deriveSecretKey", derive_secret_key)?;
    cx.export_function("underivePublicKey", underive_public_key)?;
    cx.export_function("generateSignature", generate_signature)?;
    cx.export_function("checkSignature", check_signature)?;
    cx.export_function("hashToPoint", hash_to_point)?;
    cx.export_function("hashToEC", hash_to_ec)?;
    cx.export_function("generateKeyImage", generate_key_image)?;
    cx.export_function("generateRingSignature", generate_ring_signature)?;
    cx.export_function("checkRingSignature", check_ring_signature)?;
    Ok(())
});
