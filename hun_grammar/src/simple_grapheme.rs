use crate::peg;

peg::parser!{
    pub grammar simple_grapheme() for str
    {
        //* Perform grapheme conversion.
        pub rule input() -> String =
            g: grapheme() i: input() { format!( "{}{}", &g, &i ) } /
            grapheme()

        rule grapheme() -> String =
            complex() / simple()

        rule complex() -> String =
            "bb" { "B".to_string() } /
            // "b" { "b".to_string() } /
            "ccs" { "Č".to_string() } /
            "cs" { "č".to_string() } /
            "cc" { "C".to_string() } /
            // "c" { "c".to_string() } /
            "dd" { "D".to_string() } /
            // "d" { "d".to_string() } /
            "ff" { "F".to_string() } /
            // "f" { "f".to_string() } /
            "ggy" { "Ď".to_string() } /
            "gy" { "ď".to_string() } /
            "gg" { "G".to_string() } /
            // "g" { "g".to_string() } /
            "hh" { "H".to_string() } /
            // "h" { "h".to_string() } /
            "jj" { "J".to_string() } /
            // "j" { "j".to_string() } /
            "kk" { "K".to_string() } /
            // "k" { "k".to_string() } /
            "lly" { "J".to_string() } /
            "ly" { "j".to_string() } /
            "ll" { "L".to_string() } /
            // "l" { "l".to_string() } /
            "mm" { "M".to_string() } /
            // "m" { "m".to_string() } /
            "nny" { "Ń".to_string() } /
            "ny" { "ń".to_string() } /
            "nn" { "N".to_string() } /
            // "n" { "n".to_string() } /
            "pp" { "P".to_string() } /
            // "p" { "p".to_string() } /
            "rr" { "R".to_string() } /
            // "r" { "r".to_string() } /
            "ssz" { "S".to_string() } /
            "sz" { "s".to_string() } /
            "ss" { "Š".to_string() } /
            "s" { "š".to_string() } /
            "tty" { "Ť".to_string() } /
            "ty" { "ť".to_string() } /
            "tt" { "T".to_string() } /
            // "t" { "t".to_string() } /
            "vv" { "V".to_string() } /
            // "v" { "v".to_string() } /
            "zzs" { "Ž".to_string() } /
            "zs" { "ž".to_string() } /
            "zz" { "Z".to_string() }
            // "z" { "z".to_string() } /
            // "y" { "y".to_string() }

        rule simple() -> String =
            c: $([_]) { c.to_string() }
            /*
            v: $("a" / "e" / "i" / "o" / "u" /
                 "á" / "é" / "í" / "ó" / "ú" /
                 "ö" / "ő" / "ü" / "ű" /
                 "q" / "x" / "w"
            )
            { v.to_string() }
            */
    }
}
