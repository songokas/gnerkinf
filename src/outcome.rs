use core::{fmt::Display, future::Future};
use std::io::Write;

pub struct OutlineContext<T> {
    writer: Box<dyn Write>,
    value: Vec<T>,
}

pub fn outline<T>(
    s: impl Display,
    value: Vec<T>,
    mut w: impl Write + 'static,
) -> OutlineContext<T> {
    writeln!(w, "Scenario outline {}", s).expect("writing failed");
    OutlineContext {
        writer: Box::new(w),
        value,
    }
}

impl<T> OutlineContext<T> {
    pub fn map<Closure>(mut self, fut: Closure)
    where
        Closure: Fn(T, usize),
    {
        writeln!(self.writer, "=========================").expect("writing failed");
        self.value
            .into_iter()
            .enumerate()
            .for_each(|(index, data)| fut(data, index));
        self.writer.flush().expect("outcome flush");
    }

    pub async fn mapf<Closure, Fut>(mut self, fut: Closure)
    where
        Closure: Fn(T, usize) -> Fut,
        Fut: Future<Output = ()>,
    {
        writeln!(self.writer, "=========================").expect("writing failed");
        for (index, value) in self.value.into_iter().enumerate() {
            fut(value, index).await
        }
        self.writer.flush().expect("outcome flush");
    }
}
