use std::marker::PhantomData;

/// Meta-interface for type-level interfaces.
pub trait IInterface<I> {
    /// The associated interface type.
    /// Could be defaulted to I but the feature is not supported:
    /// type Interface = I;
    type Interface;
}

/// Existential type-level wrapper for user-defined implementation types.
pub struct Wrapper<I, T> (PhantomData::<(I, T)>);

/// Instance that connects together a kind and an implementation type
/// thus creating new type-level interface
impl<I, T> IInterface<I> for Wrapper<I, T> {
  type Interface = I;
}

/// Universal evaluation mechanism
pub trait Eval<Verb, Res>{
  fn eval() -> Res;
}

/// Universal evaluation mechanism (with context)
pub trait EvalCtx<Ctx, Verb, Res>{
  fn eval_ctx(ctx: Ctx) -> Res;
}
