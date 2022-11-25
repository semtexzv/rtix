#![feature(generators, generator_trait, type_alias_impl_trait, trait_alias)]
#![allow(unused_imports, unused_macros)]
#![deny(unused_must_use)]

use std::future::Future;
use std::ops::Generator;
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Coroutine<'a, A: 'a, M: Message> = Generator<(&'a mut A, &'a mut Context<'a>), Yield=Poll<()>, Return=<M as Message>::Result>;

pub trait Message {
    type Result;
}

pub trait Handler<M: Message>: Sized {
    type Response: for<'a> Coroutine<'a, Self, M>;
    fn handle(msg: M) -> Self::Response;
}

macro_rules! wait {
    ($e:expr, $cx:expr) => {{
        let mut _fut = $e;
        loop {
            match Pin::new_unchecked(&mut _fut).poll($cx) {
                Poll::Pending => yield Poll::Pending,
                Poll::Ready(r) => break r,
            };
        }
    }};
}

macro_rules! ret {
    ($e: expr) => { {return {$e}; yield Poll::Pending; }};
}

// impl Message for i32 {
//     type Result = i32;
// }
//
// impl Handler<i32> for i32 {
//     type Response = impl for<'a> Coroutine<'a, Self, i32>;
//
//     fn handle(m: i32) -> Self::Response {
//         move |(this, cx): (&mut i32, &mut Context)| {
//             loop {
//                 let done = wait!(async move { m }, cx);
//                 *this += done;
//
//                 if *this > 100 {
//                     break;
//                 }
//             }
//
//             ret!(0)
//         }
//     }
// }