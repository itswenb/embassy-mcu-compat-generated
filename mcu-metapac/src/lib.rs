#![no_std]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![doc(html_no_source)]
#![doc = include_str!("../README.md")]

pub mod common;

#[cfg(feature = "pac")]
include!(env!("STM32_METAPAC_PAC_PATH"));

#[cfg(feature = "metadata")]
pub mod metadata {
    include!("metadata.rs");
    include!(env!("STM32_METAPAC_METADATA_PATH"));
    include!("all_chips.rs");
    include!("all_peripheral_versions.rs");
}
