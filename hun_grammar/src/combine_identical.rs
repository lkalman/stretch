use crate::peg;

peg::parser!{
    pub grammar combine_identical() for str
    {
        //* Perform geminate simplification next to consonants
        pub rule input() -> String =
            g: grapheme() i: input() { format!( "{}{}", &g, &i ) } /
            grapheme()

        rule grapheme() -> String =
            no_geminate() /
            geminate_sequence()

        rule no_geminate() -> String =
            s: $("a" / "e" / "i" / "o" / "u" /
                 "á" / "é" / "í" / "ó" / "ú" /
                 "ö" / "ő" / "ü" / "ű" /
                 "q" / "w" / "x" / "y")
            { s.to_string() }
        
        rule geminate_sequence() -> String =
            "bb" { "B".to_string() } /
            "bB" { "B".to_string() } /
            "Bb" { "B".to_string() } /
            "b" { "b".to_string() } /
            "B" { "B".to_string() } /
            "cc" { "C".to_string() } /
            "cC" { "C".to_string() } /
            "Cc" { "C".to_string() } /
            "c" { "c".to_string() } /
            "C" { "C".to_string() } /
            "čč" { "Č".to_string() } /
            "čČ" { "Č".to_string() } /
            "Čč" { "Č".to_string() } /
            "č" { "č".to_string() } /
            "Č" { "Č".to_string() } /
            "dd" { "D".to_string() } /
            "dD" { "D".to_string() } /
            "Dd" { "D".to_string() } /
            "d" { "d".to_string() } /
            "D" { "D".to_string() } /
            "ff" { "F".to_string() } /
            "fF" { "F".to_string() } /
            "Ff" { "F".to_string() } /
            "f" { "f".to_string() } /
            "F" { "F".to_string() } /
            "gg" { "G".to_string() } /
            "gG" { "G".to_string() } /
            "Gg" { "G".to_string() } /
            "g" { "g".to_string() } /
            "G" { "G".to_string() } /
            "ďď" { "Ď".to_string() } /
            "ďĎ" { "Ď".to_string() } /
            "Ďď" { "Ď".to_string() } /
            "ď" { "ď".to_string() } /
            "Ď" { "Ď".to_string() } /
            "hh" { "H".to_string() } /
            "hH" { "H".to_string() } /
            "Hh" { "H".to_string() } /
            "h" { "h".to_string() } /
            "H" { "H".to_string() } /
            "jj" { "J".to_string() } /
            "jJ" { "J".to_string() } /
            "Jj" { "J".to_string() } /
            "j" { "j".to_string() } /
            "J" { "J".to_string() } /
            "kk" { "K".to_string() } /
            "kK" { "K".to_string() } /
            "Kk" { "K".to_string() } /
            "k" { "k".to_string() } /
            "K" { "K".to_string() } /
            "l" { "l".to_string() } /
            "mm" { "M".to_string() } /
            "mM" { "M".to_string() } /
            "Mm" { "M".to_string() } /
            "m" { "m".to_string() } /
            "M" { "M".to_string() } /
            "ńń" { "Ń".to_string() } /
            "ńŃ" { "Ń".to_string() } /
            "Ńń" { "Ń".to_string() } /
            "ń" { "ń".to_string() } /
            "Ń" { "Ń".to_string() } /
            "nn" { "N".to_string() } /
            "nN" { "N".to_string() } /
            "Nn" { "N".to_string() } /
            "n" { "n".to_string() } /
            "N" { "N".to_string() } /
            "pp" { "P".to_string() } /
            "pP" { "P".to_string() } /
            "Pp" { "P".to_string() } /
            "p" { "p".to_string() } /
            "P" { "P".to_string() } /
            "rr" { "R".to_string() } /
            "rR" { "R".to_string() } /
            "Rr" { "R".to_string() } /
            "r" { "r".to_string() } /
            "R" { "R".to_string() } /
            "ss" { "S".to_string() } /
            "sS" { "S".to_string() } /
            "Ss" { "S".to_string() } /
            "s" { "s".to_string() } /
            "S" { "S".to_string() } /
            "šš" { "Š".to_string() } /
            "šŠ" { "Š".to_string() } /
            "Šš" { "Š".to_string() } /
            "š" { "š".to_string() } /
            "Š" { "Š".to_string() } /
            "tt" { "T".to_string() } /
            "tT" { "T".to_string() } /
            "Tt" { "T".to_string() } /
            "t" { "t".to_string() } /
            "T" { "T".to_string() } /
            "ťť" { "Ť".to_string() } /
            "ťŤ" { "Ť".to_string() } /
            "Ťť" { "Ť".to_string() } /
            "ť" { "ť".to_string() } /
            "Ť" { "Ť".to_string() } /
            "vv" { "V".to_string() } /
            "vV" { "V".to_string() } /
            "Vv" { "V".to_string() } /
            "v" { "v".to_string() } /
            "V" { "V".to_string() } /
            "z" { "z".to_string() } /
            "žž" { "Ž".to_string() } /
            "žŽ" { "Ž".to_string() } /
            "Žž" { "Ž".to_string() } /
            "ž" { "ž".to_string() } /
            "Ž" { "Ž".to_string() }
    }
}
