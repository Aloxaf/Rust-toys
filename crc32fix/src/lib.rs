mod errors;

pub use crate::errors::Error;
use crc::crc32;
use png::{Decoded, DecodingError, StreamingDecoder};
use std::fs;
use std::io::prelude::*;
use std::mem;

/// 储存用来校验 crc 值的数据和 crc 值本身
#[derive(Debug, Default)]
pub struct CrcData {
    pub type_str: [u8; 4],
    pub width: u32,
    pub height: u32,
    pub bits: u8,
    pub color_type: u8,
    pub compr_method: u8,
    pub filter_method: u8,
    pub interlace_method: u8,
    pub crc_val: u32,
}

impl CrcData {
    pub fn from_data(data: &[u8]) -> Result<Self, Error> {
        let mut crcdata: CrcData = Default::default();
        let mut decoder = StreamingDecoder::new();
        let mut idx = 0;

        for _ in 0..3 {
            let (len, decoded) = match decoder.update(&data[idx..]) {
                Ok(t) => t,
                Err(DecodingError::CrcMismatch { crc_val, .. }) => {
                    crcdata.crc_val = crc_val;
                    return Ok(crcdata);
                }
                Err(e @ _) => return Err(Error::ParseError(e)),
            };

            match decoded {
                Decoded::ChunkBegin(_length, type_str) => {
                    crcdata.type_str.clone_from_slice(&type_str);
                }
                Decoded::Header(width, height, bit_depth, color_type, interlaced) => {
                    crcdata.width = width;
                    crcdata.height = height;
                    crcdata.bits = bit_depth as u8;
                    crcdata.color_type = color_type as u8;
                    crcdata.interlace_method = interlaced as u8;
                }
                _ => (),
            }

            idx += len;
        }
        Err(Error::CorrectCrc)
    }

    /// 将 CrcData 转化为字节数组
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        let bwidth: [u8; 4] = unsafe { mem::transmute(self.width) };
        let bheight: [u8; 4] = unsafe { mem::transmute(self.height) };

        bytes.extend(self.type_str.iter());
        bytes.extend(bwidth.iter().rev());
        bytes.extend(bheight.iter().rev());
        bytes.extend(
            [
                self.bits,
                self.color_type,
                self.compr_method,
                self.filter_method,
                self.interlace_method,
            ]
            .iter(),
        );

        bytes
    }

    /// 爆破 crc32 值
    pub fn try_fix(&mut self) -> Result<(), ()> {
        let width = self.width;

        for i in 1..8192 {
            self.width = i;
            if self.crc_val == crc32::checksum_ieee(&self.as_bytes()) {
                return Ok(());
            }
        }
        self.width = width;
        for i in 1..8192 {
            self.height = i;
            if self.crc_val == crc32::checksum_ieee(&self.as_bytes()) {
                return Ok(());
            }
        }
        Err(())
    }
}

/// 从指定位置替换数据
pub fn replace_nbytes(src: &mut Vec<u8>, offset: usize, data: &[u8]) {
    src[offset..(data.len() + offset)].clone_from_slice(&data[..]);
}

/// 保存文件
pub fn save_file(filename: &str, data: &[u8]) -> std::io::Result<()> {
    let mut file = fs::File::create(filename)?;
    file.write_all(data)?;
    Ok(())
}
