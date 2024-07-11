use core::{fmt::Display, future::Future, pin::Pin};
use std::io::Write;

use crate::{ThenContext, ThenContextFuture};
pub struct WhenContext<T> {
    pub(crate) writer: Box<dyn Write>,
    pub(crate) value: T,
}

impl<T> WhenContext<T> {
    pub fn when<Closure, EndType>(mut self, s: impl Display, fut: Closure) -> WhenContext<EndType>
    where
        Closure: FnOnce(T) -> EndType,
    {
        writeln!(self.writer, "When {}", s).expect("writing failed");
        WhenContext {
            writer: self.writer,
            value: fut(self.value),
        }
    }

    pub async fn whenf<Closure, Fut, EndType>(
        mut self,
        s: impl Display,
        fut: Closure,
    ) -> WhenContext<EndType>
    where
        Closure: FnOnce(T) -> Fut,
        Fut: Future<Output = EndType>,
    {
        writeln!(self.writer, "When {}", s).expect("writing failed");
        WhenContext {
            writer: self.writer,
            value: fut(self.value).await,
        }
    }

    pub fn and<Closure, EndType>(mut self, s: impl Display, fut: Closure) -> WhenContext<EndType>
    where
        Closure: FnOnce(T) -> EndType,
    {
        writeln!(self.writer, "And {}", s).expect("writing failed");
        WhenContext {
            writer: self.writer,
            value: fut(self.value),
        }
    }

    pub async fn andf<Closure, Fut, EndType>(
        mut self,
        s: impl Display,
        fut: Closure,
    ) -> WhenContext<EndType>
    where
        Closure: FnOnce(T) -> Fut,
        Fut: Future<Output = EndType>,
    {
        writeln!(self.writer, "And {}", s).expect("writing failed");
        WhenContext {
            writer: self.writer,
            value: fut(self.value).await,
        }
    }

    pub async fn thenf<Closure, Fut, EndType>(
        self,
        s: impl Display,
        fut: Closure,
    ) -> ThenContext<EndType>
    where
        Closure: FnOnce(T) -> Fut,
        Fut: Future<Output = EndType>,
    {
        ThenContext {
            writer: self.writer,
            value: self.value,
        }
        .thenf(s, fut)
        .await
    }

    pub fn thenfw<Closure, Fut, EndType>(
        self,
        s: impl Display + 'static,
        fut: Closure,
    ) -> ThenContextFuture<EndType>
    where
        T: 'static,
        EndType: 'static,
        Closure: FnOnce(T) -> Fut + 'static,
        Fut: Future<Output = EndType> + 'static,
    {
        let context = ThenContext {
            writer: self.writer,
            value: self.value,
        }
        .thenf(s, fut);

        ThenContextFuture {
            context: Box::pin(context),
        }
    }

    pub fn then<Closure, EndType>(self, s: impl Display, fut: Closure) -> ThenContext<EndType>
    where
        Closure: FnOnce(T) -> EndType,
    {
        ThenContext {
            writer: self.writer,
            value: self.value,
        }
        .then(s, fut)
    }
}

#[must_use = "Context must be awaited"]
pub struct WhenContextFuture<T> {
    pub(crate) context: Pin<Box<dyn Future<Output = WhenContext<T>>>>,
}

impl<T> Future for WhenContextFuture<T> {
    type Output = WhenContext<T>;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.context.as_mut().poll(cx)
    }
}

impl<T> WhenContextFuture<T> {
    pub fn andf<Closure, Fut, EndType>(
        self,
        s: impl Display + 'static,
        fut: Closure,
    ) -> WhenContextFuture<EndType>
    where
        T: 'static,
        EndType: 'static,
        Closure: FnOnce(T) -> Fut + 'static,
        Fut: Future<Output = EndType> + 'static,
    {
        let context = async {
            let p = self.context.await;
            WhenContext {
                writer: p.writer,
                value: p.value,
            }
            .andf(s, fut)
            .await
        };

        WhenContextFuture {
            context: Box::pin(context),
        }
    }

    pub fn and<Closure, EndType>(
        self,
        s: impl Display + 'static,
        fut: Closure,
    ) -> WhenContextFuture<EndType>
    where
        T: 'static,
        EndType: 'static,
        Closure: FnOnce(T) -> EndType + 'static,
    {
        let context = async {
            let p = self.context.await;
            WhenContext {
                writer: p.writer,
                value: p.value,
            }
            .and(s, fut)
        };

        WhenContextFuture {
            context: Box::pin(context),
        }
    }

    pub fn thenf<Closure, Fut, EndType>(
        self,
        s: impl Display + 'static,
        fut: Closure,
    ) -> ThenContextFuture<EndType>
    where
        T: 'static,
        EndType: 'static,
        Closure: FnOnce(T) -> Fut + 'static,
        Fut: Future<Output = EndType> + 'static,
    {
        let context = async {
            let p = self.context.await;
            ThenContext {
                writer: p.writer,
                value: p.value,
            }
            .thenf(s, fut)
            .await
        };

        ThenContextFuture {
            context: Box::pin(context),
        }
    }

    pub fn then<Closure, EndType>(
        self,
        s: impl Display + 'static,
        fut: Closure,
    ) -> ThenContextFuture<EndType>
    where
        T: 'static,
        EndType: 'static,
        Closure: FnOnce(T) -> EndType + 'static,
    {
        let context = async {
            let p = self.context.await;
            ThenContext {
                writer: p.writer,
                value: p.value,
            }
            .then(s, fut)
        };

        ThenContextFuture {
            context: Box::pin(context),
        }
    }
}
