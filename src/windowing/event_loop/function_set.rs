use crate::context::Context;

pub struct FunctionSet<S> {
    pub(crate) init_state: fn(&mut Context) -> S,
    pub(crate) update: Vec<fn(&mut S, &mut Context)>,
    pub(crate) fixed_update: Vec<fn(&mut S, &mut Context)>,
}

impl<S> FunctionSet<S> {
    pub fn get_state(&self, ctx: &mut Context) -> S {
        let init = self.init_state;
        init(ctx)
    }

    pub fn run_update(&self, state: &mut S, ctx: &mut Context) {
        for function in self.update.iter() {
            function(state, ctx);
        }
    }

    pub fn run_fixed_update(&self, state: &mut S, ctx: &mut Context) {
        for function in self.fixed_update.iter() {
            function(state, ctx);
        }
    }

    pub fn new(init_state: fn(&mut Context) -> S) -> Self {
        Self {
            init_state,
            update: vec![],
            fixed_update: vec![],
        }
    }
}
