pub use expectest::prelude::be_ok;

pub use sql::lexer::Tokenizer;
pub use sql::parser::Parser;
pub use sql::query_executer::QueryExecuter;

describe! create_table_queries {

    it "creates single column table" {
        let mut executer = QueryExecuter::new();

        expect!(executer.execute("create table table_name (col int);".to_owned().tokenize().unwrap().parse().unwrap()))
            .to(be_ok().value("'table_name' was created".to_owned()));
    }
}