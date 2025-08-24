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
    pub r#type: usize,       // 0x0008
    pub offset: i32,         // 0x0010
    pub metadata_count: i32, // 0x0014
    pub metadata: usize,     // 0x0018 SchemaMetadataEntryData
}
