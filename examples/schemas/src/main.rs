mod raw_schema;
mod writers;

use std::{fs::File, io::Error};

use exbase::*;
use crate::{raw_schema::*, writers::{Context, ModuleWriter, RsModuleWriter}};

const OUTPUT_DIR: &str = "./output";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

fn main() -> Result<(), Error> {
    if !std::fs::exists(OUTPUT_DIR)? {
        std::fs::create_dir(OUTPUT_DIR)?;
    }
    let proc = exbase::get_process_info_list("dota2")
        .unwrap()
        .into_iter()
        .next()
        .expect("Process not found");

    let libschema = proc.get_modules()
        .unwrap()
        .into_iter()
        .find(|x| x.name() == "libschemasystem.so")
        .expect("'libschemasystem.so' not found");

    let mem = StreamMem::new(proc.pid()).unwrap();

    let mut schema = Schema::new(&mem, libschema);
    let modules = schema.read_scopes();

    for module in modules {
        let len = module.classes.len();
        let file_name = &format!("{}/{}.rs", OUTPUT_DIR, module.name());
        if len > 0 {
            let mut file = File::create(file_name)?;
            RsModuleWriter::write_module(&mut Context::new(&mem, &module, &mut file))?;
            println!("{GREEN}{file_name}{RESET}");
        }
        else {
            println!("{file_name}");
        }
    }

    Ok(())
}

pub struct Schema<'a, M> where M: MemoryAccessor {
    type_scopes_len: i32,
    type_scopes_vec: usize,
    mem: &'a M
}

pub struct TypeScope {
    module_name: String,
    pub classes: Vec<Class>
}

pub struct Class {
    raw: SchemaClassInfoData
}

#[derive(Debug)]
pub struct Enum {
    raw: SchemaEnumInfoData
}

pub struct Field {
    raw: SchemaClassFieldData
}

impl<'a, M: MemoryAccessor> Schema<'a, M> {
    pub fn new(mem: &'a M, libschema: ModuleInfo) -> Self {
        let schema_system = Self::find_schema_system(mem, libschema);

        let type_scopes_len: i32 = mem.read(schema_system + 0x1F0);
        let type_scopes_vec: usize = mem.read(schema_system + 0x1F8);

        assert_ne!(type_scopes_len, 0);

        Self {
            type_scopes_len,
            type_scopes_vec,
            mem
        }
    }

    pub fn read_scopes(&mut self) -> Vec<TypeScope> {
        let mut scopes: Vec<TypeScope> = Vec::new();
        for i in 0..self.type_scopes_len {
            let type_scope_address = self.mem.read::<usize>(self.type_scopes_vec + (i * 8) as usize);
            scopes.push(TypeScope::new(self.mem, type_scope_address));
        }
        scopes
    }

    fn find_schema_system(mem: &impl MemoryAccessor, libschema: ModuleInfo) -> usize {
        let mut buf = vec![0u8; libschema.size()];
        mem.read_buffer(&mut buf, libschema.address());

        let pat_offset = *Pattern::new("48 8D 0D ? ? ? ? 48 8D 3D ? ? ? ? E8")
            .unwrap()
            .scan(&mut buf, true)
            .iter()
            .next()
            .expect("outdated schema system pattern");

        relative_address(mem, libschema.address() + pat_offset, 10, 14)
    }
}

impl TypeScope {
    fn new(mem: &impl MemoryAccessor, address: usize) -> Self {
        let module_name = mem.read_string(address as usize + 0x08, 256);

        let mut classes: Vec<Class> = Vec::new();
        let mut enums: Vec<Enum> = Vec::new();

        let class_bindings = address + 0x560 + 0x90;
        let enum_bindings = address + 0x3600 + 0x90;

        for i in 0..256 {
            let mut node_ptr: usize = mem.read(class_bindings as usize + (i * 0x30) + 0x28);

            while node_ptr != 0 {
                let class_ptr: usize = mem.read(node_ptr as usize + 0x10);
                if class_ptr != 0 {
                    let class = Class::new(mem, class_ptr as usize);
                    classes.push(class);
                }
                node_ptr = mem.read(node_ptr as usize + 0x08);
            }
        }

        for i in 0..256 {
            let mut node_ptr: usize = mem.read(enum_bindings as usize + (i * 0x30) + 0x28);

            while node_ptr != 0 {
                let enum_ptr: usize = mem.read(node_ptr as usize + 0x10);
                if enum_ptr != 0 {
                    let r#enum = Enum::new(mem, enum_ptr as usize);
                    enums.push(r#enum);
                }
                node_ptr = mem.read(node_ptr as usize + 0x08);
            }
        }

        // for enm in enums {
        //     println!("{}", enm.read_name(mem));
        // }
        
        Self {
            module_name,
            classes,
            // enums
        }
    } 

    fn name(&self) -> String {
        self.module_name.trim_start_matches("lib").trim_end_matches(".so").to_owned()
    }
}

impl Class {
    pub fn new(mem: &impl MemoryAccessor, ptr: usize) -> Self {
        let raw = mem.read::<SchemaClassInfoData>(ptr);
        Self {
            raw
        }
    }

    pub fn read_parent(&self, mem: &impl MemoryAccessor) -> Option<String> {
        let base_class = mem.read::<SchemaBaseClassInfoData>(self.raw.base_classes);
        let parent_class = mem.read::<SchemaBaseClass>(base_class.prev);
        let r = mem.read_string(parent_class.name, 256);
        match r.is_empty() {
            true => None,
            false => Some(r)
        }
    }

    pub fn read_name(&self, mem: &impl MemoryAccessor) -> String {
        mem.read_string(self.raw.name, 256).replace(":", "_")
    }

    pub fn read_fields(&self, mem: &impl MemoryAccessor) -> Vec<Field> {
        if self.raw.field_count == 0 { return Vec::default(); }

        let mut result: Vec<Field> = Vec::new();
        for i in 0..self.raw.field_count {
            let field = mem.read::<SchemaClassFieldData>(self.raw.fields + (size_of::<SchemaClassFieldData>() * i as usize));
            result.push(Field { raw: field });
        }
        
        result
    }
}

impl Enum {
    pub fn new(mem: &impl MemoryAccessor, ptr: usize) -> Self {
        let raw = mem.read::<SchemaEnumInfoData>(ptr);
        Self {
            raw
        }
    }

    pub fn read_name(&self, mem: &impl MemoryAccessor) -> String {
        mem.read_string(self.raw.name, 256).replace(":", "_")
    }
}

impl Field {
    pub fn read_name(&self, mem: &impl MemoryAccessor) -> String {
        mem.read_string(self.raw.name, 256)
    }

    pub fn read_type_name(&self, mem: &impl MemoryAccessor) -> String {
        let r#type = mem.read::<SchemaType>(self.raw.r#type);
        let type_name = mem.read_string(r#type.name, 128).replace(" ", "");
        type_name
    }

    pub fn get_offset(&self) -> i32 {
        self.raw.offset
    }
}