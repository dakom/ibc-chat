use futures::{future::{self, Either, FusedFuture}, pin_mut, select, Future};
use gloo_timers::future::TimeoutFuture;
use anyhow::{Error, Result};
use crate::config::CONFIG;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TimeoutError {
    #[error("IBC timeout error")]
    IbcTimeout,
}

// simple one-off to race a future against the ibc timeout time
pub async fn race_against_ibc_timeout<F, T>(fut: F) -> Result<T> 
where F: Future<Output = T>
{

    let fut1 = TimeoutFuture::new(CONFIG.ibc_response_timeout_ms);
    let fut2 = fut;

    pin_mut!(fut2);

    match future::select(fut1, fut2).await {
        Either::Left(_) => Err(TimeoutError::IbcTimeout.into()),
        Either::Right((value, _)) => Ok(value),
    }
}

/// given a "future generator" function, poll until the future resolves with a value
/// or until the ibc timeout is reached
/// takes cloneable deps that get passed through to the future generator
pub async fn try_until_ibc_timeout<FutureGenerator, F, T, D>(deps: D, future_generator: FutureGenerator) -> Result<T> 
where FutureGenerator: Fn(D) -> F,
      F: Future<Output = Option<T>>,
      D: Clone
{
    let fut1 = TimeoutFuture::new(CONFIG.ibc_response_timeout_ms);
    let fut2 = async move {
        loop {
            let fut = future_generator(deps.clone());
            let result = fut.await;
            if let Some(result) = result {
                return result;
            }

            TimeoutFuture::new(CONFIG.ibc_poll_delay_ms).await;
        }
    }; 

    pin_mut!(fut2);

    match future::select(fut1, fut2).await {
        Either::Left(_) => Err(TimeoutError::IbcTimeout.into()), 
        Either::Right((value, _)) => Ok(value),
    }
}

// Taken from: https://github.com/Pauan/rust-dominator/blob/75a7af18de8fd5ae9371ec38b6826c3894a4a355/src/macros.rs#L590 
#[macro_export]
macro_rules! clone {
    ($($input:tt)*) => { $crate::__internal_clone_split!((), $($input)*) };
}
#[macro_export]
macro_rules! __internal_clone_split {
    (($($x:ident)*), $t:ident => $y:expr) => {{
        $(let $x = $x.clone();)*
        let $t = $t.clone();
        $y
    }};
    (($($x:ident)*), $t:ident, $($after:tt)*) => {
        $crate::__internal_clone_split!(($($x)* $t), $($after)*)
    };
}