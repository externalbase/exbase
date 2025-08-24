mod raw_schema;
mod writers;

use std::{fs::File, io::Error};

use exbase::*;
use crate::{raw_schema::*, writers::{Context, ModuleWriter, RsModuleWriter}};

const OUTPUT_DIR: &str = "./output";

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
        let mut file = File::create(format!("{}/{}.rs", OUTPUT_DIR, module.module_name))?;
        RsModuleWriter::write_module(&mut Context::new(&mem, &module, &mut file))?
    }

    Ok(())
}

pub struct Schema<'a, M> where M: MemoryAccessor {
    type_scopes_len: i32,
    type_scopes_vec: usize,
    mem: &'a M
}

pub struct TypeScope {
    pub module_name: String,
    pub classes: Vec<Class>
}

pub struct Class {
    raw: SchemaClassInfoData
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
        let buckets_offset = address + 0x560 + 0x90;
        for i in 0..256 {
            let mut node_ptr: usize = mem.read(buckets_offset as usize + (i * 0x30) + 0x28);

            while node_ptr != 0 {
                let class_ptr: usize = mem.read(node_ptr as usize + 0x10);
                if class_ptr != 0 {
                    let class = Class::new(mem, class_ptr as usize);
                    classes.push(class);
                }
                node_ptr = mem.read(node_ptr as usize + 0x08);
            }
        }
        Self {
            module_name,
            classes
        }
    } 
}

impl Class {
    pub fn new(mem: &impl MemoryAccessor, ptr: usize) -> Self {
        let raw = mem.read::<SchemaClassInfoData>(ptr);
        Self {
            raw
        }
    }

    pub fn read_parent(&self, mem: &impl MemoryAccessor) -> String {
        let base_class = mem.read::<SchemaBaseClassInfoData>(self.raw.base_classes);
        let parent_class = mem.read::<SchemaBaseClass>(base_class.prev);
        mem.read_string(parent_class.name, 256)
    }

    pub fn read_name(&self, mem: &impl MemoryAccessor) -> String {
        mem.read_string(self.raw.name, 256).replace(":", "_")
    }
}