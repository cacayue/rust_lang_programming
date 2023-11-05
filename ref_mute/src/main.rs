use::std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;


fn main() {
    let mut table = Table::new();
    table.insert("1".to_string(), vec!["1_1".to_string(), "1_2".to_string(), "1_3".to_string()]);
    table.insert("2".to_string(), vec!["2_1".to_string(), "2_2".to_string(), "2_3".to_string()]);
    show(&table);
    assert_eq!(table["1"][0], "1_1");
}

fn show(table: &Table){
    for (artist, works) in table {
        println!(" ");
        println!("works by {}:", artist);
        for work in works {
            print!("  {}",work);
        }
    }
}


