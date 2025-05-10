use crate::model_traits::TableIdent;

pub mod create_table;
use crate::Syntax;
use crate::writers::TableWriter;

mod create_indexes;
pub use create_indexes::write as create_index;

pub fn drop_table(syntax: Syntax, table: &TableIdent) -> String {
    let tablename = TableWriter::new(syntax).write(table);
    format!("DROP TABLE {}", tablename)
}

mod rename_column;
pub use rename_column::write as rename_column;

mod alter_column_type;
pub use alter_column_type::write_down as alter_column_type_down;
pub use alter_column_type::write_up as alter_column_type_up;

mod drop_column;
pub use drop_column::write as drop_column;

mod add_column;
pub use add_column::write as add_column;
