#![allow(unused)]

// https://github.com/a2x/cs2-dumper/blob/main/src/source2/schema_system/schema_class_info_data.rs

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SchemaClassInfoData {
    pub base: usize,                // 0x0000 SchemaClassInfoData
    pub name: usize,                // 0x0008
    pub module_name: usize,         // 0x0010
    pub size: i32,                  // 0x0018
    pub field_count: i16,           // 0x001C
    pub static_metadata_count: i16, // 0x001E
    pad_0020: [u8; 0x2],            // 0x0020
    pub align_of: u8,               // 0x0022
    pub has_base_class: u8,         // 0x0023
    pub total_class_size: i16,      // 0x0024
    pub derived_class_size: i16,    // 0x0026
    pub fields: usize,              // 0x0028 [SchemaClassFieldData]
    pad_0038: [u8; 0x8],            // 0x0030
    pub base_classes: usize,        // 0x0038 SchemaBaseClassInfoData
    pub static_metadata: usize,     // 0x0040
    pub type_scope: usize,          // 0x0050
    pub r#type: usize,              // 0x0058 SchemaType
    pad_0060: [u8; 0x10],           // 0x0060
}
// https://github.com/a2x/cs2-dumper/blob/main/src/source2/schema_system/schema_base_class_info_data.rs

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SchemaBaseClassInfoData {
    pad_0000: [u8; 0x18], // 0x0000
    pub prev: usize,      // 0x0018 SchemaBaseClass
}

// https://github.com/a2x/cs2-dumper/blob/main/src/source2/schema_system/schema_base_class_info_data.rs

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SchemaBaseClass {
    pad_0000: [u8; 0x10], // 0x0000
    pub name: usize,      // 0x0010 ReprCString
}

// https://github.com/a2x/cs2-dumper/blob/main/src/source2/schema_system/schema_class_field_data.rs

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SchemaClassFieldData {
    pub name: usize,         // 0x0000 ReprCString
    pub r#type: usize,       // 0x0008 SchemaType
    pub offset: i32,         // 0x0010
    pub metadata_count: i32, // 0x0014
    pub metadata: usize,     // 0x0018 SchemaMetadataEntryData
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SchemaType {
    pad_0000: [u8; 0x8],                   // 0x0000
    pub name: usize,                       // 0x0008 ReprCString
    pub type_scope: usize,                 // 0x0010 SchemaSystemTypeScope
    pub type_category: SchemaTypeCategory, // 0x0018
    pub atomic_category: SchemaAtomicCategory, // 0x0019
                                           // pub value: SchemaTypeUnion,                // 0x0020
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum SchemaTypeCategory {
    BuiltIn = 0,
    Ptr,
    Bitfield,
    FixedArray,
    Atomic,
    DeclaredClass,
    DeclaredEnum,
    None,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum SchemaAtomicCategory {
    Basic = 0,
    T,
    CollectionOfT,
    TF,
    TT,
    TTF,
    I,
    None,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SchemaEnumInfoData {
    pub base: usize,                // 0x0000 SchemaEnumInfoData
    pub name: usize,                // 0x0008 ReprCString
    pub module_name: usize,         // 0x0010 ReprCString
    pub size: u8,                   // 0x0018
    pub align_of: u8,               // 0x0019
    pad_001a: [u8; 0x2],            // 0x001A
    pub enum_count: u16,            // 0x001C
    pub static_metadata_count: u16, // 0x001E
    pub enums: usize,               // 0x0020 [SchemaEnumeratorInfoData]
    pub static_metadata: usize,     // 0x0028 SchemaMetadataEntryData
    pub type_scope: usize,          // 0x0030 SchemaSystemTypeScope
    pub min_enum_value: i64,        // 0x0038
    pub max_enum_value: i64,        // 0x0040
}
