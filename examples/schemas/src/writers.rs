use exbase::MemoryAccessor;
use std::io::{Error, Write};

use crate::{Class, TypeScope};

pub struct Context<'a, M, O>
where
    M: MemoryAccessor,
    O: Write,
{
    mem: &'a M,
    module: &'a TypeScope,
    out: &'a mut O,
}

impl<'a, M, O> Context<'a, M, O>
where
    M: MemoryAccessor,
    O: Write,
{
    pub fn new(mem: &'a M, module: &'a TypeScope, out: &'a mut O) -> Self {
        Self { mem, module, out }
    }
}

pub trait ModuleWriter<'a, M: MemoryAccessor, O: Write> {
    fn start(ctx: &mut Context<'a, M, O>) -> Result<(), Error>;
    fn end(ctx: &mut Context<'a, M, O>) -> Result<(), Error>;

    fn write_module(ctx: &mut Context<'a, M, O>) -> Result<(), Error> {
        Self::start(ctx)?;
        for class in &ctx.module.classes {
            Self::write_class(class, ctx)?
        }
        Self::end(ctx)?;
        Ok(())
    }

    fn write_class(class: &Class, ctx: &mut Context<'a, M, O>) -> Result<(), Error>;
}

pub struct RsModuleWriter;

impl<'a, M: MemoryAccessor, O: Write> ModuleWriter<'a, M, O> for RsModuleWriter {
    fn start(ctx: &mut Context<'a, M, O>) -> Result<(), Error> {
        write!(ctx.out, "#![allow(non_upper_case_globals, unused)]\n")?;
        Ok(())
    }

    fn end(_ctx: &mut Context<'a, M, O>) -> Result<(), Error> {
        // write!(ctx.out, "// end\n")
        Ok(())
    }

    fn write_class(class: &Class, ctx: &mut Context<'a, M, O>) -> Result<(), Error> {
        let mem = ctx.mem;
        let class_name = class.read_name(mem);

        let fields = class.read_fields(mem);
        let parent = class.read_parent(mem);
        if fields.len() > 0 {
            if let Some(parent) = parent {
                write!(ctx.out, "// Parent: {}\n", parent)?;
            }
            write!(ctx.out, "mod {} {{\n", class_name)?;

            for field in fields {
                let field_name = field.read_name(mem);
                let type_name = field.read_type_name(mem);
                let offset = field.get_offset();
                //\tpub const {FIELD_NAME}: {TYPE} = {OFFSET}; // {TYPE NAME}
                write!(ctx.out,"\tpub const {field_name}: usize = 0x{offset:x}; // {type_name}\n")?;
            }

            write!(ctx.out, "}}\n")?;
        }
        else {
            // write!(ctx.out, "mod {} {{ }} // {}\n", class_name, parent)?;
        }

        Ok(())
    }
}
