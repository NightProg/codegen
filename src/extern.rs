use crate::{Formatter, Function};
use std::fmt::Write;
#[derive(Debug, Clone)]
pub enum ExternSymbol {
    Function(Function),
}

/// Defines an extern block.
#[derive(Debug, Clone)]
pub struct Extern {
    abi: Option<String>,
    symbols: Vec<ExternSymbol>
}


impl Extern {

    /// Return a new extern block.
    pub fn new() -> Self {
        Extern {
            abi: None,
            symbols: Vec::new()
        }
    }

    /// Set the ABI for the extern block.
    pub fn abi(&mut self, abi: &str) -> &mut Self {
        self.abi = Some(abi.to_string());
        self
    }

    /// Return a new extern function with the given name.
    pub fn function(&mut self, name: &str) -> &mut Function {
        let function = Function::new(name);
        self.symbols.push(ExternSymbol::Function(function));
        match self.symbols.last_mut().unwrap() {
            ExternSymbol::Function(function) => function.no_body(),
        }
    }


    /// Push a function to the extern block.
    pub fn push_function(&mut self, function: Function) -> &mut Self {
        assert!(function.body.is_none(), "Extern functions cannot have bodies");
        self.symbols.push(ExternSymbol::Function(function));
        self
    }


    /// Formats the extern block using the given formatter.
    pub fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if let Some(abi) = &self.abi {
            writeln!(f, "extern \"{}\"", abi)?;
        } else {
            writeln!(f, "extern ")?;
        }

        f.block(|x| {
            let mut res = Ok(());
            for symbol in &self.symbols {
                match symbol {
                    ExternSymbol::Function(function) => {
                        res = function.fmt(true, x)
                    }
                }
            }

            res
        })
    }
}