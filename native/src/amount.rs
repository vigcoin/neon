use cryptonote_raw_crypto::amount::Amount;
use neon::prelude::*;

pub fn get_penalized(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let amount = cx.argument::<JsNumber>(0)?.value() as u64;
    let median_size = cx.argument::<JsNumber>(1)?.value() as usize;
    let current_block_size = cx.argument::<JsNumber>(2)?.value() as usize;
    Ok(cx.number(Amount::get_penalized(amount, median_size, current_block_size) as f64))
}