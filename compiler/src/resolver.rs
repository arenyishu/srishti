use crate::ast::Program;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

pub struct ModuleResolver {
    root_dir: PathBuf,
    modules: HashMap<String, Program>,
    visited: HashSet<String>,
}

impl ModuleResolver {
    pub fn new(root_dir: PathBuf) -> Self {
        Self {
            root_dir,
            modules: HashMap::new(),
            visited: HashSet::new(),
        }
    }

    pub fn resolve(&mut self, entry_point: &Path) -> Result<Program, String> {
        self.resolve_module(entry_point)
    }

    fn resolve_module(&mut self, path: &Path) -> Result<Program, String> {
        let canonical = match path.canonicalize() {
            Ok(p) => p.to_string_lossy().to_string(),
            Err(_) => return Err(format!("Could not find module at {:?}", path)),
        };

        if self.visited.contains(&canonical) {
            return Err(format!("Circular dependency detected involving {:?}", path));
        }

        self.visited.insert(canonical.clone());

        let source = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read module at {:?}: {}", path, e))?;

        let mut lexer = crate::lexer::Lexer::new(&source);
        let tokens = lexer.tokenize().map_err(|e| format!("Lex error in {:?}: {}", path, e))?;

        let mut parser = crate::parser::Parser::new(tokens);
        let program = parser.parse().map_err(|e| format!("Parse error in {:?}: {}", path, e))?;

        // Recursively resolve imports
        for import in &program.imports {
            let import_path = self.resolve_import_path(&import.path, path)?;
            self.resolve_module(&import_path)?;
        }

        self.modules.insert(canonical.clone(), program.clone());
        self.visited.remove(&canonical);

        Ok(program)
    }

    fn resolve_import_path(&self, import_path: &str, current_file: &Path) -> Result<PathBuf, String> {
        if import_path.starts_with("std/") {
            let std_dir = self.root_dir.join("std");
            let name = import_path.trim_start_matches("std/");
            Ok(std_dir.join(format!("{}.srishti", name)))
        } else if import_path.starts_with("./") || import_path.starts_with("../") {
            let parent_dir = current_file.parent().unwrap_or(Path::new("."));
            Ok(parent_dir.join(import_path).with_extension("srishti"))
        } else {
            let modules_dir = self.root_dir.join("srishti_modules");
            Ok(modules_dir.join(import_path).with_extension("srishti"))
        }
    }
}
