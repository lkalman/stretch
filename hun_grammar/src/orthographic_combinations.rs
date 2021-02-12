use crate::peg;

peg::parser!{
    pub grammar orthographic_combinations() for str
    {
        /**
        /// Convert into geminates combinations where the first element
        /// is a coronal stop, and the second is a coronal/palatal
        /// continuant:
        /// dentialveolar [d, t] + palatal approximate [j] -> ggy, tty
        /// dentialveolar [d, t] + dentialveolar slit [sz, z] -> cc, dz
        /// dentialveolar [d, t] + coronal slit [s, zs] -> ccs, dzs
        /// palatal [gy, ggy, ty, tty] + palatal approximate [j] -> ggy, tty
        /// dentialveolar lateral [l] + palatal approximate [j] -> jj
        /// palatal nasal [ny, nny] + palatal approximate [j] -> nny
        /// dentialveolar nasal [n] + palatal approximate [j] -> nny
        */
        pub rule input() -> String =
            g: grapheme() i: input() { format!( "{}{}", &g, &i ) } /
            grapheme()

        rule grapheme() -> String =
            combination() / other()

        rule combination() -> String = 
            "Dj" { "Ď".to_string() } /
            "DJ" { "Ď".to_string() } /
            "dj" { "Ď".to_string() } /
            "dJ" { "Ď".to_string() } /
            "Tj" { "Ť".to_string() } /
            "TJ" { "Ť".to_string() } /
            "tj" { "Ť".to_string() } /
            "tJ" { "Ť".to_string() } /
            "Ts" { "C".to_string() } /
            "TS" { "C".to_string() } /
            "ts" { "C".to_string() } /
            "tS" { "C".to_string() } /
            "Tš" { "Č".to_string() } /
            "TŠ" { "Č".to_string() } /
            "tš" { "Č".to_string() } /
            "tŠ" { "Č".to_string() } /
            "ďj" { "Ď".to_string() } /
            "Ďj" { "Ď".to_string() } /
            "ďJ" { "Ď".to_string() } /
            "ĎJ" { "Ď".to_string() } /
            "ťj" { "Ť".to_string() } /
            "Ťj" { "Ť".to_string() } /
            "ťJ" { "Ť".to_string() } /
            "ŤJ" { "Ť".to_string() } /
            "lj" { "J".to_string() } /
            "Lj" { "J".to_string() } /
            "lJ" { "J".to_string() } /
            "LJ" { "J".to_string() } /
            "nj" { "Ń".to_string() } /
            "Nj" { "Ń".to_string() } /
            "nJ" { "Ń".to_string() } /
            "NJ" { "Ń".to_string() } /
            "ńj" { "Ń".to_string() } /
            "Ńj" { "Ń".to_string() } /
            "ńJ" { "Ń".to_string() } /
            "ŃJ" { "Ń".to_string() }
            
        rule other() -> String =
            c:$([_]) { c.to_string() }
    }
}
