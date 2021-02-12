trait Trait<A>
{
    fn num( &self ) -> A;
}

struct Num
{
    num: usize,
}

impl Trait<usize> for Num
{
    fn num( &self ) -> usize
    {
        let Num { num: n } = self;
        *n
    }
}

fn main()
{
    let n = Num { num: 5 };
    println!( "{}", n.num() );
}
