use crate::context::{Context, FixedContext};

pub type Init<S> = fn(&mut Context) -> S;
pub type Update<S> = fn(&mut S, &mut Context);
pub type FixedUpdate<S> = fn(&mut S, &FixedContext);

pub struct FunctionSet<S> {
    pub(crate) init_state: Init<S>,
    pub(crate) update: Vec<Update<S>>,
    pub(crate) fixed_update: Vec<FixedUpdate<S>>,
}

impl<S> FunctionSet<S> {
    pub fn get_state(&self, ctx: &mut Context) -> S {
        let init = self.init_state;
        init(ctx)
    }

    pub fn run_update(&self, state: &mut S, ctx: &mut Context) {
        for function in &self.update {
            function(state, ctx);
        }
    }

    pub fn run_fixed_update(&self, state: &mut S, ctx: &FixedContext) {
        for function in &self.fixed_update {
            function(state, ctx);
        }
    }

    pub fn new(init_state: Init<S>) -> Self {
        Self {
            init_state,
            update: vec![],
            fixed_update: vec![],
        }
    }
}
