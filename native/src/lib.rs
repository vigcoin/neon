#[macro_use]
extern crate neon;
extern crate cryptonote_wallet;

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
            wallet.load(filename, password);

            Ok(wallet)
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
