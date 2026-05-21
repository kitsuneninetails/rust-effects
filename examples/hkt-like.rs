use rust_effects::prelude::*;

pub fn foo<'a, M>(input: impl Monad<String, u32, MonadOut = M>) -> M
where
    M: Monad<u32, ()> + Monoid + Applicative<u32>,
{
    Monad::bind(input, |a| {
        if a.len() % 2 == 1 {
            M::pure(a.len() as u32)
        } else {
            M::empty()
        }
    })
}

pub fn bar<F>(input: impl Functor<u32, String, FunctorOut = F>) -> F
where
    F: Functor<String, ()> + Monoid + 'static,
{
    Functor::fmap(
        input,
        |a| {
            if a > 3 { a.to_string() } else { "".to_string() }
        },
    )
}

#[tokio::main]
async fn main() {
    println!(
        "Should be '' (odd <= 3): {}",
        bar(foo(Some("dog".to_string()))).map_or("No".to_string(), |s| s.to_string())
    );
    println!(
        "Should be no (even): {}",
        bar(foo(Some("crow".to_string()))).map_or("No".to_string(), |s| s.to_string())
    );
    println!(
        "Should be (5) (odd > 3): {}",
        bar(foo(Some("raven".to_string()))).map_or("No".to_string(), |s| s.to_string())
    );
    println!(
        "Should be No (None): {}",
        bar(foo(None::<String>)).map_or("No".to_string(), |s| s.to_string())
    );

    println!(
        "Should be list(5, ''): {:?}",
        bar(foo(vec!["donkey", "raven", "fox"]
            .into_iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()))
    );
    println!(
        "Should be '' (Future even = 0 < 3): {:?}",
        bar(foo(CFuture::lazy("donkey".to_string()))).await
    );
    println!(
        "Should be (5): {:?}",
        bar(foo(CFuture::lazy("tiger".to_string()))).await
    );

    let startv = vec!["dog", "cat", "fox", "crow", "raven", "pigeon", "donkey"];
    let to_s = lift_m1![Vec](str::to_string);
    let v1 = to_s(startv);
    let v2: Vec<u32> = bind(v1, |i| {
        vec![(i.len() as u32), (i.matches("o").count() as u32)]
    });
    let v3: Vec<u32> = Monad::<u32, u32>::bind(v2, |i| if i <= 4 { vec![i] } else { vec![] });
    println!("FREE output: {:?}", v3);
}
