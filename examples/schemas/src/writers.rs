use std::io::{Error, Write};
use exbase::MemoryAccessor;

use crate::{Class, TypeScope};

pub struct Context<'a, M, O> where M: MemoryAccessor, O: Write {
    mem: &'a M,
    module: &'a TypeScope,
    out: &'a mut O
}

impl<'a, M, O> Context<'a, M, O> where M: MemoryAccessor, O: Write {
    pub fn new(mem: &'a M, module: &'a TypeScope, out: &'a mut O) -> Self {
        Self {
            mem,
            module,
            out,
        }
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
        write!(ctx.out, "// start\n")
    }

    fn end(ctx: &mut Context<'a, M, O>) -> Result<(), Error> {
        write!(ctx.out, "// end\n")
    }

    fn write_class(class: &Class, ctx: &mut Context<'a, M, O>) -> Result<(), Error> {
        let class_name = class.read_name(ctx.mem);
        // Inheritance: Object -> MarshalByRefObject -> Component -> Control -> ScrollableControl -> ContainerControl -> Form
        let parent = class.read_parent(ctx.mem);
        write!(ctx.out, "// Parent: {}\n", parent)?;
        write!(ctx.out, "mod {} {{\n", class_name)?;
        write!(ctx.out, "   // todo\n")?;
        write!(ctx.out, "}}\n")
    }
}