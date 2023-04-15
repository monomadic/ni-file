//! Functionality relating to [`NISound`] documents.
//!
/*
- `Repository`
    - `Item`
        - `ItemHeader` ('hsin') // NI::SOUND::ItemHeader::write(NI::GP::Stream&)
            @ u64 FrameSize
            @ u32
            @ u32 DomainID 0x6e697368 'hsin'
            @ u32
            @ u32
            @ uuid method.NI::SOUND::ItemUuid.write_NI::GP::Stream__const
        - `ItemFrameStack`
            - `ItemFrame` (Size, DomainID, ItemID)
                @ u64 FrameSize +0
                @ u32 DomainID 0x4e495344 "DSIN" +0x8
                @ u32 ItemID +0xc(12)
                @ u32 Version +0x10(16)
                .getStreamFrameSizeInBytes() -> 0x14(20)
                .getFrameSize()
                .isFrameForBase() { DomainID != 'DSIN' || ItemID != 1 }
        - `SubtreeItem` 0x73 'DSIN' = 44 53 49 4E 73 00 00 00
            .compressData()
            .decompressData()
*/

mod header;
pub use header::ItemHeader;

mod item;
pub use item::Item;
mod item_frame;
mod item_frame_stack;
pub mod items;
mod preset;
mod property;
mod repository;

pub use item_frame::{app_id::AuthoringApplication, domain_id::*, item_id::ItemID};
pub use repository::NISound;
