use expectest::prelude::be_equal_to;

use sql::catalog_manager::{LockBasedCatalogManager, Table, Column};
use sql::lexer::{Tokenizer, IntoTokenizer};
use sql::parser::{QueryParser, IntoQueryParser};
use sql::parser::ast::InsertQuery;
use sql::parser::ast::Statement::Insert;
use sql::parser::ast::Value;
use sql::parser::ast::ValueSource;
use sql::parser::ast::Type;

#[test]
fn populates_columns_for_insert_query() {
    let catalog_manager = LockBasedCatalogManager::default();

    catalog_manager.add_table(Table::new("table2"));
    catalog_manager.add_column_to("table2", Column::new("col1", Type::Int));
    catalog_manager.add_column_to("table2", Column::new("col2", Type::Int));
    catalog_manager.add_column_to("table2", Column::new("col3", Type::Int));

    expect!(String::from("insert into table2 values (1, 2, 3);").into_tokenizer().tokenize().into_parser().parse().populate(&catalog_manager))
        .to(
            be_equal_to(
                Insert(
                    InsertQuery::new(
                        "table2",
                        vec!["col1", "col2", "col3"],
                        ValueSource::Row(vec![Value::num("1"), Value::num("2"), Value::num("3")])
                    )
                )
            )
        );
}

#[test]
fn populate_only_missed_column() {
    let catalog_manager = LockBasedCatalogManager::default();

    catalog_manager.add_table(Table::new("table_1"));
    catalog_manager.add_column_to("table_1", Column::with_default("col1", Type::Int, "1"));
    catalog_manager.add_column_to("table_1", Column::new("col2", Type::Int));

    expect!(String::from("insert into table_1 (col2) values (2);").into_tokenizer().tokenize().into_parser().parse().populate(&catalog_manager))
        .to(
            be_equal_to(
                Insert(
                    InsertQuery::new(
                        "table_1",
                        vec!["col2", "col1"],
                        ValueSource::Row(vec![Value::num("2"), Value::num("1")])
                    )
                )
            )
        );
}

#[test]
fn populate_default_value_for_different_types() {
    let catalog_manager = LockBasedCatalogManager::default();

    catalog_manager.add_table(Table::new("table_2"));
    catalog_manager.add_column_to("table_2", Column::with_default("col1", Type::Int, "1"));
    catalog_manager.add_column_to("table_2", Column::new("col2", Type::Int));
    catalog_manager.add_column_to("table_2", Column::with_default("col3", Type::VarChar(3), "str"));

    expect!(String::from("insert into table_2 (col2) values (2);").into_tokenizer().tokenize().into_parser().parse().populate(&catalog_manager))
        .to(
            be_equal_to(
                Insert(
                    InsertQuery::new(
                        "table_2",
                        vec!["col2", "col1", "col3"],
                        ValueSource::Row(vec![Value::num("2"), Value::num("1"), Value::str("str")])
                    )
                )
            )
        );
}