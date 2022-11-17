#[macro_export]
macro_rules! filter_sql {
    ($table:expr) => {
        format!("select * from {}", $table)
    };
    ($table:expr, $($condiction:expr),* ) => {
        {
            let conds = vec![
                $($condiction , )+
            ];
            format!("{}{}", format!("select * from {} where ", $table), conds.join(" & "))
        }
    };
}

#[cfg(test)]
mod test_marcos {
    #[test]
    fn select_all_sql_should_be_correct() {
        assert_eq!(
            filter_sql!("TestTable"),
            String::from("select * from TestTable")
        );
    }

    #[test]
    fn select_with_filter_sql_should_be_correct() {
        assert_eq!(
            filter_sql!("TestTable", "time=1", "done = 2"),
            String::from("select * from TestTable where time=1 & done = 2")
        );
    }
}
