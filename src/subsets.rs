fn get_bit_at( input: usize, n: usize ) -> bool
{
    if n < 32 {
        input & (1 << n) != 0
    } else {
        false
    }
}

pub fn subsets<T: Clone>( vec: &Vec<T> ) -> Vec<Vec<T>>
{
    let mut result = Vec::new();
    for i in 0..usize::pow( 2, vec.len() as u32 )
    {
        let mut subs = Vec::new();
        for j in 0..vec.len()
        {
            if get_bit_at( i, j )
            {
                subs.push( vec[j].clone() )
            }
        }
        result.push( subs );
    }
    result
}

