use crate::peg;

peg::parser!{
    pub grammar obstruent_voice() for str
    {
        //* Perform obstruent voice assimilation.
        pub rule input() -> String =
            g: grapheme() i: input() { format!( "{}{}", &g, &i ) } /
            grapheme()

        rule grapheme() -> String =
            assimilating() / other() // non_assimilating()

        rule assimilating() -> String =
            "B" &voiceless_obstruent() { "p".to_string() } /
            "B" { "B".to_string() } /
            "b" &voiceless_obstruent() { "p".to_string() } /
            "b" { "b".to_string() } /
            "Č" &voiced_obstruent() { "dž".to_string() } /
            "Č" { "Č".to_string() } /
            "č" &voiced_obstruent() { "dž".to_string() } /
            "č" { "č".to_string() } /
            "C" &voiced_obstruent() { "dz".to_string() } /
            "C" { "C".to_string() } /
            "c" &voiced_obstruent() { "dz".to_string() } /
            "c" { "c".to_string() } /
            "d" &voiceless_obstruent() { "t".to_string() } /
            "d" { "d".to_string() } /
            "D" &voiceless_obstruent() { "t".to_string() } /
            "D" { "D".to_string() } /
            "F" &voiced_obstruent() { "v".to_string() } /
            "F" { "F".to_string() } /
            "f" &voiced_obstruent() { "v".to_string() } /
            "f" { "f".to_string() } /
            "G" &voiceless_obstruent() { "k".to_string() } /
            "G" { "G".to_string() } /
            "g" &voiceless_obstruent() { "k".to_string() } /
            "g" { "g".to_string() } /
            "Ď" &voiceless_obstruent() { "ť".to_string() } /
            "Ď" { "Ď".to_string() } /
            "ď" &voiceless_obstruent() { "ť".to_string() } /
            "ď" { "ď".to_string() } /
            "K" &voiced_obstruent() { "g".to_string() } /
            "K" { "K".to_string() } /
            "k" &voiced_obstruent() { "g".to_string() } /
            "k" { "k".to_string() } /
            "P" &voiced_obstruent() { "b".to_string() } /
            "P" { "P".to_string() } /
            "p" &voiced_obstruent() { "b".to_string() } /
            "p" { "p".to_string() } /
            "S" &voiced_obstruent() { "z".to_string() } /
            "S" { "S".to_string() } /
            "s" &voiced_obstruent() { "z".to_string() } /
            "s" { "s".to_string() } /
            "Š" &voiced_obstruent() { "ž".to_string() } /
            "Š" { "Š".to_string() } /
            "š" &voiced_obstruent() { "ž".to_string() } /
            "š" { "š".to_string() } /
            "Ť" &voiced_obstruent() { "ď".to_string() } /
            "ť" &voiced_obstruent() { "ď".to_string() } /
            "T" &voiced_obstruent() { "d".to_string() } /
            "T" { "T".to_string() } /
            "t" &voiced_obstruent() { "d".to_string() } /
            "t" { "t".to_string() } /
            "V" &voiceless_obstruent() { "f".to_string() } /
            "V" { "V".to_string() } /
            "v" &voiceless_obstruent() { "f".to_string() } /
            "v" { "v".to_string() } /
            "Ž" &voiceless_obstruent() { "š".to_string() } /
            "Ž" { "Ž".to_string() } /
            "ž" &voiceless_obstruent() { "š".to_string() } /
            "ž" { "ž".to_string() } /
            "Z" &voiceless_obstruent() { "s".to_string() } /
            "Z" { "Z".to_string() } /
            "z" &voiceless_obstruent() { "s".to_string() } /
            "z" { "z".to_string() }

        rule other() -> String =
            c: $[_] { c.to_string() }
        /*
            v: $("a" / "e" / "i" / "o" / "u" /
                 "á" / "é" / "í" / "ó" / "ú" /
                 "ö" / "ő" / "ü" / "ű" / "H" /
                 "h" / "J" / "j" / "L" / "l" /
                 "M" / "m" / "N" / "n" / "ń" /
                 "Ń" / "q" / "R" / "r" / "w" /
                 "x" / "y")
            { v.to_string() }
        */
        
        rule voiceless_obstruent() =
            &"Č" /
            &"č" /
            &"C" /
            &"c" /
            &"F" /
            &"f" /
            &"H" /
            &"h" /
            &"K" /
            &"k" /
            &"P" /
            &"p" /
            &"S" /
            &"s" /
            &"Š" /
            &"š" /
            &"Ť" /
            &"ť" /
            &"T" /
            &"t"

        rule voiced_obstruent() =
            &"B" /
            &"b" /
            &"D" /
            &"d" /
            &"G" /
            &"g" /
            &"Ď" /
            &"ď" /
            &"Ž" /
            &"ž" /
            &"Z" /
            &"z"
    }
}
