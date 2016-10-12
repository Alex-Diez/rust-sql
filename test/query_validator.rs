use expectest::prelude::{be_ok, be_err};

use sql::lexer::tokenize;
use sql::parser::parse;
use sql::query_typer::type_inferring;
use sql::query_validator::validate;
use sql::ast::{Type, ValidatedStatement};
use sql::ast::create_table::{CreateTableQuery, ColumnTable};
use sql::catalog_manager::LockBasedCatalogManager;

#[test]
fn validate_create_table_qury() {
    let catalog_manager = LockBasedCatalogManager::default();

    expect!(
        tokenize("create table table1 (col1 integer);")
            .and_then(parse)
            .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
            .and_then(|statement| validate(catalog_manager.clone(), statement))
    ).to(
        be_ok().value(
            ValidatedStatement::Create(
                CreateTableQuery::new(
                    "table1",
                    vec![ColumnTable::new("col1", Type::Integer, false, None, true, None)]
                )
            )
        )
    );
}

#[test]
fn validate_create_already_existed_table() {
    let catalog_manager = LockBasedCatalogManager::default();
    catalog_manager.add_table("table1");

    expect!(
        tokenize("create table table1 (col1 integer);")
            .and_then(parse)
            .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
            .and_then(|statement| validate(catalog_manager.clone(), statement))
    ).to(be_err().value(String::from("Table <table1> already exists")));
}

#[test]
fn validate_create_table_with_two_similar_columns() {
    let catalog_manager = LockBasedCatalogManager::default();

    expect!(
        tokenize("create table table1(col1 integer, col1 integer);")
            .and_then(parse)
            .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
            .and_then(|statement| validate(catalog_manager.clone(), statement))
    ).to(be_err().value(String::from("Column <col1> is already defined in <table1>")));
}

#[test]
fn validate_create_table_with_foreign_key() {
    let catalog_manager = LockBasedCatalogManager::default();
    catalog_manager.add_table("table1");
    catalog_manager.add_column_to("table1", ("col1", Type::Integer, None));

    expect!(
        tokenize("create table table2 (col2 integer primary key, col3 integer foreign key references table1 (col1));")
            .and_then(parse)
            .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
            .and_then(|statement| validate(catalog_manager.clone(), statement))
    ).to(
        be_ok().value(
            ValidatedStatement::Create(
                CreateTableQuery::new(
                    "table2",
                    vec![
                        ColumnTable::new("col2", Type::Integer, true, None, false, None),
                        ColumnTable::new("col3", Type::Integer, false, Some((String::from("table1"), String::from("col1"))), true, None)
                    ]
                )
            )
        )
    );
}