
use std::io::Error as IoError;

use flv_future_aio::bytes::Bytes;
use flv_future_aio::bytes::BytesMut;
use flv_future_aio::fs::AsyncFileSlice;

use crate::{ Encoder, Version };


pub enum StoreValue {
    Bytes(Bytes),
    FileSlice(AsyncFileSlice),
}

impl std::fmt::Debug for StoreValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            StoreValue::Bytes(bytes) => write!(f, "StoreValue:Bytes with len: {}", bytes.len()),
            StoreValue::FileSlice(slice) => write!(f, "StoreValue:FileSlice: {:#?}", slice),
        }
    }
}

pub trait FileWrite: Encoder {
    fn file_encode(
        &self,
        src: &mut BytesMut,
        _data: &mut Vec<StoreValue>,
        version: Version,
    ) -> Result<(), IoError> {
        self.encode(src, version)
    }
}

impl<M> FileWrite for Vec<M>
where
    M: FileWrite,
{
    fn file_encode(
        &self,
        src: &mut BytesMut,
        data: &mut Vec<StoreValue>,
        version: Version,
    ) -> Result<(), IoError> {
        let len: i32 = self.len() as i32;
        len.encode(src, version)?;
        for v in self {
            v.file_encode(src, data, version)?;
        }
        Ok(())
    }
}
