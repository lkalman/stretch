use crate::peg;

peg::parser!{
    pub grammar simplify_geminates() for str
    {
        //* Perform geminate simplification next to consonants
        pub rule input( cons: &mut bool ) -> Vec<String> =
            grapheme( cons )+

        rule grapheme( cons: &mut bool ) -> String =
            v: vowel() { *cons = false; v } /
            c: consonant( cons ) { *cons = true; c }

        rule consonant( cons: &mut bool ) -> String =
            geminate( cons ) / simple()

        rule vowel() -> String =
            s: $("a" / "e" / "i" / "o" / "u"
                 / "á" / "é" / "í" / "ó" / "ú"
                 / "ö" / "ő" / "ü" / "ű")
        { s.to_string() }
        
        rule simple() -> String =
            "b" { "b".to_string() } /
            "č" { "č".to_string() } /
            "c" { "c".to_string() } /
            "d" { "d".to_string() } /
            "f" { "f".to_string() } /
            "g" { "g".to_string() } /
            "ď" { "ď".to_string() } /
            "h" { "h".to_string() } /
            "j" { "j".to_string() } /
            "k" { "k".to_string() } /
            "l" { "l".to_string() } /
            "m" { "m".to_string() } /
            "ń" { "ń".to_string() } /
            "n" { "n".to_string() } /
            "p" { "p".to_string() } /
            "q" { "q".to_string() } /
            "r" { "r".to_string() } /
            "s" { "s".to_string() } /
            "š" { "š".to_string() } /
            "ť" { "ť".to_string() } /
            "t" { "t".to_string() } /
            "v" { "v".to_string() } /
            "w" { "w".to_string() } /
            "x" { "x".to_string() } /
            "y" { "y".to_string() } /
            "ž" { "ž".to_string() } /
            "z" { "z".to_string() }

        rule geminate( cons: &mut bool ) -> String =
            "B" &consonant( cons ) { "b".to_string() } /
            "B"
        {
            if *cons { "b".to_string() }
            else { "B".to_string() }
        } /
            "Č" &consonant( cons ) { "č".to_string() } /
            "Č"
        {
            if *cons { "č".to_string() }
            else { "Č".to_string() }
        } /
            "C" &consonant( cons ) { "c".to_string() } /
            "C"
        {
            if *cons { "c".to_string() }
            else { "C".to_string() }
        } /
            "D" &consonant( cons ) { "d".to_string() } /
            "D"
        {
            if *cons { "d".to_string() }
            else { "D".to_string() }
        } /
            "F" &consonant( cons ) { "f".to_string() } /
            "F"
        {
            if *cons { "f".to_string() }
            else { "F".to_string() }
        } /
            "G" &consonant( cons ) { "g".to_string() } /
            "G"
        {
            if *cons { "g".to_string() }
            else { "G".to_string() }
        } /
            "Ď" &consonant( cons ) { "ď".to_string() } /
            "Ď"
        {
            if *cons { "ď".to_string() }
            else { "Ď".to_string() }
        } /
            "H" &consonant( cons ) { "h".to_string() } /
            "H"
        {
            if *cons { "h".to_string() }
            else { "H".to_string() }
        } /
            "J" &consonant( cons ) { "j".to_string() } /
            "J"
        {
            if *cons { "j".to_string() }
            else { "J".to_string() }
        } /
            "K" &consonant( cons ) { "k".to_string() } /
            "K"
        {
            if *cons { "k".to_string() }
            else { "K".to_string() }
        } /
            "L" &consonant( cons ) { "l".to_string() } /
            "L"
        {
            if *cons { "l".to_string() }
            else { "L".to_string() }
        } /
            "M" &consonant( cons ) { "m".to_string() } /
            "M"
        {
            if *cons { "m".to_string() }
            else { "M".to_string() }
        } /
            "Ń" &consonant( cons ) { "ń".to_string() } /
            "Ń"
        {
            if *cons { "ń".to_string() }
            else { "Ń".to_string() }
        } /
            "N" &consonant( cons ) { "n".to_string() } /
            "N"
        {
            if *cons { "n".to_string() }
            else { "N".to_string() }
        } /
            "P" &consonant( cons ) { "p".to_string() } /
            "P"
        {
            if *cons { "p".to_string() }
            else { "P".to_string() }
        } /
            "R" &consonant( cons ) { "r".to_string() } /
            "R"
        {
            if *cons { "r".to_string() }
            else { "R".to_string() }
        } /
            "S" &consonant( cons ) { "s".to_string() } /
            "S"
        {
            if *cons { "s".to_string() }
            else { "S".to_string() }
        } /
            "Š" &consonant( cons ) { "š".to_string() } /
            "Š"
        {
            if *cons { "š".to_string() }
            else { "Š".to_string() }
        } /
            "Ť" &consonant( cons ) { "ť".to_string() } /
            "Ť"
        {
            if *cons { "ť".to_string() }
            else { "Ť".to_string() }
        } /
            "T" &consonant( cons ) { "t".to_string() } /
            "T"
        {
            if *cons { "t".to_string() }
            else { "T".to_string() }
        } /
            "V" &consonant( cons ) { "v".to_string() } /
            "V"
        {
            if *cons { "v".to_string() }
            else { "V".to_string() }
        } /
            "Ž" &consonant( cons ) { "ž".to_string() } /
            "Ž"
        {
            if *cons { "ž".to_string() }
            else { "Ž".to_string() }
        } /
            "Z" &consonant( cons ) { "z".to_string() } /
            "Z"
        {
            if *cons { "z".to_string() }
            else { "Z".to_string() }
        }
    }
}
