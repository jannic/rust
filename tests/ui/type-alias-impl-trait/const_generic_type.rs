//@edition: 2021
//@revisions: infer no_infer

#![feature(type_alias_impl_trait)]
type Bar = impl std::fmt::Display;
//[no_infer]~^ ERROR: unconstrained opaque type

async fn test<const N: crate::Bar>() {
    //~^ ERROR: `Bar` is forbidden as the type of a const generic parameter
    #[cfg(infer)]
    let x: u32 = N;
}

fn main() {}
