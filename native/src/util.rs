use neon::prelude::*;

pub fn get_u8_32_array(data: &Vec<u8>) -> [u8; 32] {
    let mut fixed_data : [u8; 32] = [0; 32];
    for i in 0..32 {
        fixed_data[i] = data[i];
    }
    fixed_data
}

pub fn get_u8_64_array(data: &Vec<u8>) -> [u8; 64] {
    let mut fixed_data : [u8; 64] = [0; 64];
    for i in 0..64 {
        fixed_data[i] = data[i];
    }
    fixed_data
}

pub fn get_buffer(cx: &mut neon::context::CallContext<'_, neon::types::JsObject>, idx: i32) -> Vec<u8>{
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

// pub fn get_buffer_array(cx: &mut neon::context::CallContext<'_, neon::types::JsObject>, idx: i32) -> Result<Vec<[u8; 32]>, Throw>{
//     let ba: Handle<JsArray> = cx.argument::<JsArray>(2)?;
//     let vec: Vec<Handle<JsValue>> = ba.to_vec(&mut cx)?;
//     to_buffer_array(&mut cx, &vec);
// }

pub fn to_buffer_array(cx: &mut neon::context::CallContext<'_, neon::types::JsObject>, bufs: &Vec<Handle<JsValue>>) -> Vec<[u8;32]>{
    let mut buffer_array: Vec<[u8; 32]> = vec![];
    // Iterate over each item in the array
    for i in 0..bufs.len() {
        // Each item is a generic `JsValue`, downcast it to a `JsBuffer`
        let buf : Handle<JsBuffer> = bufs[i].downcast::<JsBuffer>().expect("Fail to download JsBuffer!");
        // Borrow the internal buffer from the `JsBuffer` as `&[u8]`
        // `borrow_mut` is also available for a mutable reference
        let buf = cx.borrow(&buf, |buf| buf.as_slice::<u8>());
        let mut buffer: Vec<u8> = vec![];
        for i in 0..buf.len() {
            buffer.push(buf[i]);
        }
        buffer_array.push(get_u8_32_array(&buffer));
    }
    buffer_array
}


pub fn get_hash(cx: &mut neon::context::CallContext<'_, neon::types::JsObject>, idx: i32) -> [u8; 32]{
    let data = get_buffer(cx, idx);
    get_u8_32_array(&data)
}

pub fn get_signature(cx: &mut neon::context::CallContext<'_, neon::types::JsObject>, idx: i32) -> [u8; 64]{
    let data = get_buffer(cx, idx);
    get_u8_64_array(&data)
}
