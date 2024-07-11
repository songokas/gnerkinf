use core::{fmt::Display, future::Future, pin::Pin};
use std::io::Write;

use crate::{WhenContext, WhenContextFuture};

pub fn given<T, F>(s: impl Display, callback: F, mut w: impl Write) -> GivenContext<T>
where
    F: FnOnce() -> T,
{
    writeln!(w, "Given {}", s).expect("writing failed");
    GivenContext {
        writer: Box::new(w),
        value: callback(),
    }
}

pub fn given_data<T>(s: impl Display, value: T, mut w: impl Write + 'static) -> GivenContext<T> {
    writeln!(w, "Given {}", s).expect("writing failed");
    GivenContext {
        writer: Box::new(w),
        value,
    }
}

pub fn givenf<EndType, Callback, Fut>(
    s: impl Display + 'static,
    callback: Callback,
    w: impl Write + 'static,
) -> GivenContextFuture<EndType>
where
    Callback: FnOnce() -> Fut,
    Fut: Future<Output = EndType> + 'static,
    EndType: 'static,
{
    let context = given_future(s, callback(), w);
    GivenContextFuture {
        context: Box::pin(context),
    }
}

pub fn given_dataf<T>(
    s: impl Display + 'static,
    value: T,
    w: impl Write + 'static,
) -> GivenContextFuture<T>
where
    T: 'static,
{
    let context = givenf(s, || async { value }, w);
    GivenContextFuture {
        context: Box::pin(context),
    }
}

pub struct GivenContext<T> {
    writer: Box<dyn Write>,
    value: T,
}

impl<T> GivenContext<T> {
    pub fn when<Closure, EndType>(self, s: impl Display, fut: Closure) -> WhenContext<EndType>
    where
        Closure: FnOnce(T) -> EndType,
    {
        WhenContext {
            writer: self.writer,
            value: self.value,
        }
        .when(s, fut)
    }

    pub async fn whenf<Closure, Fut, EndType>(
        self,
        s: impl Display,
        fut: Closure,
    ) -> WhenContext<EndType>
    where
        Closure: FnOnce(T) -> Fut,
        Fut: Future<Output = EndType>,
    {
        WhenContext {
            writer: self.writer,
            value: self.value,
        }
        .whenf(s, fut)
        .await
    }

    pub fn and<Closure, EndType>(mut self, s: impl Display, fut: Closure) -> GivenContext<EndType>
    where
        Closure: FnOnce(T) -> EndType,
    {
        writeln!(self.writer, "And {}", s).expect("writing failed");
        GivenContext {
            writer: self.writer,
            value: fut(self.value),
        }
    }

    pub async fn andf<Closure, Fut, EndType>(
        mut self,
        s: impl Display,
        fut: Closure,
    ) -> GivenContext<EndType>
    where
        Closure: FnOnce(T) -> Fut,
        Fut: Future<Output = EndType>,
    {
        writeln!(self.writer, "And {}", s).expect("writing failed");
        GivenContext {
            writer: self.writer,
            value: fut(self.value).await,
        }
    }
}

#[must_use = "Context must be awaited"]
pub struct GivenContextFuture<T> {
    context: Pin<Box<dyn Future<Output = GivenContext<T>>>>,
}

impl<T> Future for GivenContextFuture<T> {
    type Output = GivenContext<T>;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.context.as_mut().poll(cx)
    }
}

impl<T> GivenContextFuture<T> {
    pub fn andf<Closure, Fut, EndType>(
        self,
        s: impl Display + 'static,
        fut: Closure,
    ) -> GivenContextFuture<EndType>
    where
        T: 'static,
        EndType: 'static,
        Closure: FnOnce(T) -> Fut + 'static,
        Fut: Future<Output = EndType> + 'static,
    {
        let context = async {
            let p = self.context.await;
            GivenContext {
                writer: p.writer,
                value: p.value,
            }
            .andf(s, fut)
            .await
        };

        GivenContextFuture {
            context: Box::pin(context),
        }
    }

    pub fn and<Closure, EndType>(
        self,
        s: impl Display + 'static,
        fut: Closure,
    ) -> GivenContextFuture<EndType>
    where
        T: 'static,
        Closure: FnOnce(T) -> EndType + 'static,
    {
        let context = async {
            let p = self.context.await;
            GivenContext {
                writer: p.writer,
                value: p.value,
            }
            .and(s, fut)
        };

        GivenContextFuture {
            context: Box::pin(context),
        }
    }

    pub fn when<Closure, EndType>(
        self,
        s: impl Display + 'static,
        fut: Closure,
    ) -> WhenContextFuture<EndType>
    where
        T: 'static,
        Closure: FnOnce(T) -> EndType + 'static,
    {
        let context = async {
            let p = self.context.await;
            WhenContext {
                writer: p.writer,
                value: p.value,
            }
            .when(s, fut)
        };
        WhenContextFuture {
            context: Box::pin(context),
        }
    }

    pub fn whenf<Closure, Fut, EndType>(
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
            .whenf(s, fut)
            .await
        };

        WhenContextFuture {
            context: Box::pin(context),
        }
    }
}

async fn given_future<EndType>(
    s: impl Display,
    fut: impl Future<Output = EndType>,
    mut w: impl Write + 'static,
) -> GivenContext<EndType> {
    writeln!(w, "Given {}", s).expect("writing failed");
    GivenContext {
        writer: Box::new(w),
        value: fut.await,
    }
}
