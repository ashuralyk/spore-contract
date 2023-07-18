#![no_std]

extern crate alloc;
use molecule::prelude::{ Entity, Builder };
use alloc::string::String;
use alloc::vec::Vec;
use crate::generated::spore_types::{Bytes, BytesOpt, Bool, SporeData};

pub mod generated;

impl Into<Bytes> for &[u8] {
    fn into(self) -> Bytes {
        let len = self.len();
        let mut vec: Vec<u8> = Vec::with_capacity(4 + len);
        vec.extend_from_slice(&(len as u32).to_le_bytes()[..]);
        vec.extend_from_slice(self);
        Bytes::new_unchecked(Bytes::from_slice(vec.as_slice()).unwrap().as_bytes())
    }
}

impl Into<BytesOpt> for &[u8] {
    fn into(self) -> BytesOpt {
        let len = self.len();
        let mut vec: Vec<u8> = Vec::with_capacity(4 + len);
        vec.extend_from_slice(&(len as u32).to_le_bytes()[..]);
        vec.extend_from_slice(self);
        BytesOpt::new_unchecked(BytesOpt::from_slice(vec.as_slice()).unwrap().as_bytes())
    }
}

#[derive(Debug)]
pub struct NativeNFTData {
    pub content_type: String,
    pub content: Vec<u8>,
    pub cluster: Option<String>,
}

impl From<NativeNFTData> for generated::spore_types::SporeData {
    fn from(data: NativeNFTData) -> Self {
        let content: Bytes = data.content.as_slice().into();
        let content_type: Bytes = data.content_type.as_bytes().into();
        let group = match data.cluster {
            Some(group) => group.as_bytes().into(),
            None => BytesOpt::default(),
        };
         SporeData::new_builder()
            .content(content)
            .content_type(content_type)
            .cluster(group)
            .build()
    }
}


impl From::<generated::spore_types::Bool> for bool {
    fn from(value: generated::spore_types::Bool) -> bool {
         match value.as_slice().first().unwrap_or(&0) {
             0 => false,
             1 => true,
             _ => false,
         }
    }
}


impl From::<generated::spore_types::BoolOpt> for bool {
    fn from(value: generated::spore_types::BoolOpt) -> bool {
        if value.is_none() {
            return false
        }

        bool::from(Bool::from_slice(value.as_slice()).unwrap())
    }
}