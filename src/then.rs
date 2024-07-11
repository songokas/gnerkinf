use core::{fmt::Display, future::Future, pin::Pin};
use std::io::Write;

pub struct ThenContext<T> {
    pub(crate) writer: Box<dyn Write>,
    pub(crate) value: T,
}

impl<T> ThenContext<T> {
    pub fn then<Closure, EndType>(mut self, s: impl Display, fut: Closure) -> ThenContext<EndType>
    where
        Closure: FnOnce(T) -> EndType,
    {
        writeln!(self.writer, "Then {}", s).expect("writing failed");
        ThenContext {
            writer: self.writer,
            value: fut(self.value),
        }
    }

    pub async fn thenf<Closure, Fut, EndType>(
        mut self,
        s: impl Display,
        fut: Closure,
    ) -> ThenContext<EndType>
    where
        Closure: FnOnce(T) -> Fut,
        Fut: Future<Output = EndType>,
    {
        writeln!(self.writer, "Then {}", s).expect("writing failed");
        ThenContext {
            writer: self.writer,
            value: fut(self.value).await,
        }
    }

    pub fn and<Closure, EndType>(mut self, s: impl Display, fut: Closure) -> ThenContext<EndType>
    where
        Closure: FnOnce(T) -> EndType,
    {
        writeln!(self.writer, "And {}", s).expect("writing failed");
        ThenContext {
            writer: self.writer,
            value: fut(self.value),
        }
    }

    pub async fn andf<Closure, Fut, EndType>(
        mut self,
        s: impl Display,
        fut: Closure,
    ) -> ThenContext<EndType>
    where
        Closure: FnOnce(T) -> Fut,
        Fut: Future<Output = EndType>,
    {
        writeln!(self.writer, "And {}", s).expect("writing failed");
        ThenContext {
            writer: self.writer,
            value: fut(self.value).await,
        }
    }
}

#[must_use = "Context must be awaited"]
pub struct ThenContextFuture<T> {
    pub(crate) context: Pin<Box<dyn Future<Output = ThenContext<T>>>>,
}

impl<T> Future for ThenContextFuture<T> {
    type Output = ThenContext<T>;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.context.as_mut().poll(cx)
    }
}

impl<T> ThenContextFuture<T> {
    pub fn andf<Closure, Fut, EndType>(
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
            .andf(s, fut)
            .await
        };

        ThenContextFuture {
            context: Box::pin(context),
        }
    }

    pub fn and<Closure, EndType>(
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
            .and(s, fut)
        };

        ThenContextFuture {
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
}
