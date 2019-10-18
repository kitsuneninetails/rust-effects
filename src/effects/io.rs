use crate::prelude::*;

struct IO<'a, X> {
    fut: ConcreteFuture<'a, X>,
}

