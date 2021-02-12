use crate::nom_peg;

extern crate tabular as tab;
use crate::tab::tabular::Tabular;
use crate::tab::table::Table;

let tabular = grammar!
{
    tabular: impl Tabular = "=" => { <Table as Tabular>::new() }
}
