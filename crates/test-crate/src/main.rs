trait Foo {}
trait Bar {}
trait Constrain {
    type Output;
}

struct Unit;

impl<T, U> Foo for T
where
    T: Constrain<Output = U>,
    U: Bar,
{
}

impl Constrain for Unit {
    type Output = Unit;
}

fn needs_foo<T: Foo>() {}

fn main() {
    needs_foo::<Unit>();
}