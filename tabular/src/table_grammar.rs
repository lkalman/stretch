use crate::peg;
use crate::table;

// 
peg::parser!{
    pub grammar table_grammar() for str {

        pub rule table() -> table::Table<'input> =
            space()* t: tiers() space()* { table::Table { tiers: t } }

        rule tiers() -> Vec<table::Tier<'input>> =
            ( t1: tier() space()* "," space()* ts: tiers()
              { let mut v = vec![t1]; v.append( &mut ts.clone() ); v }
              /
              t: tier() { vec![t] } )

        rule tier() -> table::Tier<'input> =
            s: slots() ( space()* ";" )?
        { table::Tier { slots: s } }

        rule slots() -> Vec<table::Slot<'input>> =
            ( s1: slot() space()+ ss: slots()
              { let mut v = s1.clone();
                v.append( &mut ss.clone() );
                v }
              / s: slot() { s } )
            
        rule slot() -> Vec<table::Slot<'input>> =
            ( size: integer() space()+ e: element()
              {
                  let mut v = Vec::new();
                  for i in 0..size
                  {
                      let mut slot = table::Slot::new( e );
                      if i == 0
                      {
                          slot.set_left_aligned( true );
                      }
                      if i == size - 1
                      {
                          slot.set_right_aligned( true );
                      }
                      v.push( slot );
                  }
                  v
              }
              / e: element()
              {
                  let mut slot = table::Slot::new( e );
                  slot.set_left_aligned( true );
                  slot.set_right_aligned( true );
                  vec![slot]
              } )
            
        rule element() -> table::Element<'input> =
            (
                s: $( characters() )
                { table::Element::Element( s ) }
                /
                "_" { table::Element::Null }
            )
            
        rule characters() -> &'input str
            = cs: $( (['a'..='z'] / ['A'..='Z'] / "-" )+ )
                      { cs }
        
        rule integer() -> usize 
            = i: $(['0'..='9']+) { i.parse::<usize>().unwrap() }

        rule space()
            = quiet!{ " " / "\t" / "\n" } / expected!( "space" )
    }
}

