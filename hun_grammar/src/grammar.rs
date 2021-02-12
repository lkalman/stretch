use crate::peg;

peg::parser!{
    pub grammar hun_grammar() for str {
        //* Parser for breaking down hungarian character strings into
        //* graphemes, `input()` returns a vector of graphemes as strings.
        
        pub rule input() -> Vec<&'input str>
            = ( complex_grapheme() / simple_grapheme() )+
        
        rule complex_grapheme() -> &'input str
            = s: ($("ccs") / $("cs") / $("ggy") / $("gy")
                  / $("lly") / $("ly") / $("nny") / $("ny")
                  / $("ssz") / $("sz") / $("tty") / $("ty")
                  / $("zzs") / $("zs")
            )
        
        rule simple_grapheme() ->  &'input str
            = s: $("a" / "e" / "i" / "o" / "u"
                   / "á" / "é" / "í" / "ó" / "ú"
                   / "ö" / "ő" / "ü" / "ű"
                   / "b" / "c" / "d" / "f" / "g"
                   / "h" / "j" / "k" / "l" / "m"
                   / "n" / "p" / "q" / "r" / "s"
                   / "t" / "v" / "w" / "x" / "y" / "z")
    }
}
