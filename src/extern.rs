use crate::{Formatter, Function};

#[derive(Debug, Clone)]
pub enum ExternsSymbol {
    Function(Function),
}


#[derive(Debug, Clone)]
pub struct Extern {
    abi: Option<String>,
    symbols: Vec<ExternsSymbol>
}


impl Extern {
    pub fn new() -> Self {
        Extern {
            abi: None,
            symbols: Vec::new()
        }
    }

    pub fn abi(&mut self, abi: &str) -> &mut Self {
        self.abi = Some(abi.to_string());
        self
    }

    pub fn function(&mut self, function: Function) -> &mut Self {
        assert!(function.body.is_none(), "Extern functions cannot have bodies");
        self.symbols.push(ExternsSymbol::Function(function));
        self
    }

    pub fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if let Some(abi) = &self.abi {
            writeln!(f, "extern \"{}\" {{", abi)?;
        } else {
            writeln!(f, "extern {{")?;
        }

        for symbol in &self.symbols {
            write!(f, "\t")?;
            match symbol {
                ExternsSymbol::Function(function) => {
                    function.fmt(false, f)?;
                }
            }
        }

        writeln!(f, "}}")
    }
}