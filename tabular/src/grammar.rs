use crate::peg;

peg::parser!{
    pub grammar tabular_grammar() for str {
        pub rule input() -> Vec<Vec<(&'input str, &'input str)>>
            = line()+
        rule line() -> Vec<(&'input str, &'input str)>
            = ( "\n" / space() )* s: slot()+ "\n"
                { s }
        rule slot() -> (&'input str, &'input str)
            = s: string() ( space() / "\n" )* i: integer() space()*
                { (s, i) }
        rule string() -> &'input str
            = "\"" s: $( characters() ) "\"" 
                { s }
        rule characters() -> &'input str
            = !"\"" cs: $( (['a'..='z'] / ['A'..='Z'] / "-" / " " / "\t" / "\\\"" )* ) { cs }
        rule integer() -> &'input str
            = $(['0'..='9']+)
        rule space()
            = " " / "\t"
    }
}

