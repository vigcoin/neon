#[macro_use]
extern crate neon;
extern crate cryptonote_wallet;
extern crate hex;

use cryptonote_wallet::{Wallet};

use neon::prelude::*;

#[no_mangle]
pub extern fn __cxa_pure_virtual() {
    loop{};
}

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
    Ok(())
});
