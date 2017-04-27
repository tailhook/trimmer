use typed_arena::Arena;

use vars::Variable;


pub struct Context<'a> {
    arena: Arena<Box<Variable+'a>>,
}

impl<'a> Context<'a> {
    pub fn new() -> Context<'a> {
        Context {
            arena: Arena::new(),
        }
    }
    pub fn memoize<T: Variable + 'a>(&'a self, value: T) -> &'a Variable {
        &**self.arena.alloc(Box::new(value))
    }
}
