use rust_lapper::{Interval, Lapper};

use crate::compiler::error::CompilerError;

use super::{
    ir::{FunctionTemplateId, VariableId},
    typ::TypeId,
    Location, Program,
};
use std::{
    collections::{HashMap, LinkedList},
    ops::Range,
};

#[derive(Debug, Default)]
pub struct Scope {
    pub variable_lookup: HashMap<String, VariableId>,
    pub type_lookup: HashMap<String, TypeId>,
    pub function_template_lookup: HashMap<String, FunctionTemplateId>,
}

impl Scope {
    // Only keep global variables
    pub fn create_child(program: &Program) -> Self {
        let scope = program.get_scope();

        Self {
            variable_lookup: HashMap::new(),
            function_template_lookup: scope
                .function_template_lookup
                .clone()
                .into_iter()
                .collect::<HashMap<_, _>>(),
            type_lookup: scope
                .type_lookup
                .clone()
                .into_iter()
                .collect::<HashMap<_, _>>(),
            // variable_lookup: scope
            //     .variable_lookup
            //     .clone()
            //     .into_iter()
            //     // .filter(|(_, variable_id)| {
            //     //     program.variables.get(*variable_id).unwrap().scope == Scoping::Global
            //     // })
            //     .collect::<HashMap<_, _>>(),
        }
    }
}

pub type ScopeId = usize;
pub type ScopeRange = (Range<Location>, ScopeId);
type Iv = Interval<Location, ScopeId>;

#[derive(Debug)]
struct ScopeTree {
    scope_id: ScopeId,
    range: Range<Location>,
    scopes: Vec<ScopeTree>,
}

impl Program {
    pub fn get_scope_mut(&mut self) -> &mut Scope {
        self.scopes.get_mut(self.scope_id).unwrap()
    }

    pub fn get_scope(&self) -> &Scope {
        self.scopes.get(self.scope_id).unwrap()
    }

    /// Used when entering a new scope, makes sure that variables are properly scoped.
    /// Inner function should return the new builder and the range of the location of the scope.
    pub fn with_scope<F, T>(&mut self, inner: F) -> Result<T, CompilerError>
    where
        F: FnOnce(&mut Self) -> Result<(T, Range<Location>), CompilerError>,
    {
        let old_scope_id = self.scope_id;
        let scope_id = self.scopes.len();
        let scope = Scope::create_child(self);
        self.scopes.push(scope);
        self.scope_id = scope_id;

        let (result, location) = inner(self)?;

        self.scope_ranges.push((location, self.scope_id));

        self.scope_id = old_scope_id;

        Ok(result)
    }

    fn get_scope_tree(
        parent: &ScopeTree,
        scope_ranges: &mut std::iter::Peekable<std::slice::Iter<ScopeRange>>,
    ) -> Option<ScopeTree> {
        let next = scope_ranges.peek()?;

        if next.0.start > parent.range.end {
            return None;
        }

        let next = scope_ranges.next().unwrap();

        let mut child = ScopeTree {
            scope_id: next.1,
            range: next.0.clone(),
            scopes: vec![],
        };

        while let Some(scope) = Program::get_scope_tree(&child, scope_ranges) {
            child.scopes.push(scope);
        }

        Some(child)
    }

    fn get_scope_ranges(parent: &ScopeTree) -> LinkedList<ScopeRange> {
        let mut ranges = LinkedList::new();
        let mut cursor = parent.range.start;

        for scope in &parent.scopes {
            ranges.push_back((cursor..scope.range.start, parent.scope_id));
            let mut child_ranges = Program::get_scope_ranges(scope);
            ranges.append(&mut child_ranges);
            cursor = scope.range.end;
        }

        ranges.push_back((cursor..parent.range.end, parent.scope_id));

        ranges
    }

    pub fn get_scope_lookup(&mut self) -> Lapper<Location, ScopeId> {
        self.scope_ranges.sort_by(|a, b| a.0.end.cmp(&b.0.end));
        self.scope_ranges.sort_by(|a, b| a.0.start.cmp(&b.0.start));

        let mut scope_ranges = self.scope_ranges.iter().peekable();
        let root = scope_ranges.next().unwrap();

        let mut tree = ScopeTree {
            scope_id: root.1,
            range: root.0.clone(),
            scopes: vec![],
        };

        while let Some(scope) = Program::get_scope_tree(&tree, &mut scope_ranges) {
            tree.scopes.push(scope);
        }

        let ranges = Program::get_scope_ranges(&tree);

        Lapper::new(
            ranges
                .iter()
                .map(|range| Iv {
                    start: range.0.start,
                    stop: range.0.end,
                    val: range.1,
                })
                .collect(),
        )
    }
}
