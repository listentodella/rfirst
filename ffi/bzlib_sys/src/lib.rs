// 生成的 bindings 代码根据c/c++代码生成
// 里面有一些不符合rust约定, 我们不让编译器报警
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

use anyhow::{anyhow, Result};
use std::mem;

mod bindings;
pub use bindings::*;

// 正常情况下应该创建另一个crate来撰写和ffi对接的接口,
// 这样有助于把高阶和低阶接口分离

//这里为了方便,就直接写在lib.rs

pub fn compress(input: &[u8]) -> Result<Vec<u8>> {
    let mut output = vec![0u8; input.len()];
    unsafe {
        let mut stream = mem::zeroed::<bz_stream>();
        let ret = BZ2_bzCompressInit(&mut stream as *mut _, 1, 0, 0);
        if ret != BZ_OK as _ {
            return Err(anyhow!("failed to intialize"));
        }

        // 传入input/output进行压缩
        stream.next_in = input.as_ptr() as *mut _;
        stream.avail_in = input.len() as _;
        stream.next_out = output.as_mut_ptr() as *mut _;
        stream.avail_out = output.len() as _;

        let ret = BZ2_bzCompress(&mut stream as *mut _, BZ_FINISH as _);
        if ret != BZ_STREAM_END as _ {
            return Err(anyhow!("failed to compress"));
        }

        // 压缩完成, 释放资源
        let ret = BZ2_bzCompressEnd(&mut stream as *mut _);
        if ret != BZ_OK as _ {
            return Err(anyhow!("failed to end compress"));
        }

        Ok(output)
    }
}

pub fn decomposes(input: &[u8]) -> Result<Vec<u8>> {
    let mut output = vec![0u8; input.len()];
    unsafe {
        let mut stream = mem::zeroed::<bz_stream>();
        let ret = BZ2_bzDecompressInit(&mut stream as *mut _, 0, 0);
        if ret != BZ_OK as _ {
            return Err(anyhow!("failed to intialize"));
        }

        // 传入input/output进行解压
        stream.next_in = input.as_ptr() as *mut _;
        stream.avail_in = input.len() as _;
        stream.next_out = output.as_mut_ptr() as *mut _;
        stream.avail_out = output.len() as _;

        let ret = BZ2_bzDecompress(&mut stream as *mut _);
        if ret != BZ_STREAM_END as _ {
            return Err(anyhow!("failed to decompress"));
        }

        // 解压完成, 释放资源
        let ret = BZ2_bzDecompressEnd(&mut stream as *mut _);
        if ret != BZ_OK as _ {
            return Err(anyhow!("failed to end decompress"));
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compression_decompression_should_work() {
        let input = include_str!("bindings.rs").as_bytes();
        let compressed = compress(input).unwrap();
        let decompressed = decomposes(&compressed).unwrap();

        assert_eq!(input, &decompressed);
    }
}
