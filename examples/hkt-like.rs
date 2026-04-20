use futures::future::ready;
use rust_effects::{
    CFuture, applicative::Applicative, functor::Functor, monad::Monad, monoid::Monoid,
};

pub fn foo<'a, M>(input: impl Monad<'a, String, u32, M = M>) -> M
where
    M: Monad<'a, u32, ()> + Monoid + Applicative<'a, u32>,
{
    Monad::bind(input, |a| {
        if a.len() % 2 == 1 {
            M::pure(a.len() as u32)
        } else {
            M::empty()
        }
    })
}

pub fn bar<'a, F>(input: impl Functor<'a, u32, String, F = F>) -> F
where
    F: Functor<'a, String, ()> + Monoid,
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
        bar(foo::<Option<_>>(Some("crow".to_string()))).map_or("No".to_string(), |s| s.to_string())
    );
    println!(
        "Should be (5) (odd > 3): {}",
        bar(foo::<Option<_>>(Some("raven".to_string())))
            .map_or("No".to_string(), |s| s.to_string())
    );
    println!(
        "Should be No (None): {}",
        bar(foo::<Option<_>>(Option::<String>::None)).map_or("No".to_string(), |s| s.to_string())
    );

    println!(
        "Should be list(5, ''): {:?}",
        bar(foo::<Vec<_>>(
            vec!["donkey", "raven", "fox"]
                .into_iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
        ))
    );
    println!(
        "Should be '' (Future even = 0 < 3): {:?}",
        bar(foo::<CFuture<_>>(CFuture::new_fut(ready(
            "donkey".to_string()
        ))))
        .await
    );
    println!(
        "Should be (5): {:?}",
        bar(foo::<CFuture<_>>(CFuture::new_fut(ready(
            "tiger".to_string()
        ))))
        .await
    );

    let startv = vec!["dog", "cat", "fox", "crow", "raven", "pigeon", "donkey"];
    let v1: Vec<String> = Functor::<&str, String>::fmap(startv, |s| s.to_string());
    let v2: Vec<u32> = Monad::<String, u32>::bind(v1, |i| {
        vec![(i.len() as u32), (i.matches("o").count() as u32)]
    });
    let v3: Vec<u32> = Monad::<u32, u32>::bind(v2, |i| if i <= 4 { vec![i] } else { vec![] });
    println!("FREE output: {:?}", v3);
}
