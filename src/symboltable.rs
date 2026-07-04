use std::collections::HashMap;

use crate::ast::Literal;

pub struct SymbolTable {
    scopes: Vec<Scope>, // arena of all scopes, never removed
    current: usize,     // index of the active scope
}

struct Scope {
    parent: Option<usize>,
    symbols: HashMap<String, Symbol>,
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub ty: Literal,
    pub scope: usize, // which scope this was declared in
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            scopes: vec![Scope {
                parent: None,
                symbols: HashMap::new(),
            }],
            current: 0,
        }
    }

    pub fn print(&self) {
        for (idx, scope) in self.scopes.iter().enumerate() {
            println!("Scope {}", idx);
            for (name, symbol) in &scope.symbols {
                println!("  {}: {:?}", name, symbol);
            }
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(Scope {
            parent: Some(self.current),
            symbols: HashMap::new(),
        });
        self.current = self.scopes.len() - 1;
    }

    pub fn exit_scope(&mut self) {
        // just move the cursor back up; nothing is deleted
        self.current = self.scopes[self.current].parent.unwrap();
    }

    pub fn define(&mut self, name: String, ty: Literal) {
        let scope = self.current;
        self.scopes[scope]
            .symbols
            .insert(name, Symbol { ty, scope });
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        let mut scope = Some(self.current);
        while let Some(idx) = scope {
            if let Some(sym) = self.scopes[idx].symbols.get(name) {
                return Some(sym);
            }
            scope = self.scopes[idx].parent;
        }
        None
    }
}
