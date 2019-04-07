#[macro_use]
extern crate neon;
extern crate cryptonote_wallet;

use cryptonote_wallet::{Wallet};

use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
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
    }
}

register_module!(mut cx, {
    cx.export_function("hello", hello);
    cx.export_class::<JsWallet>("Wallet")?;
    Ok(())
});
