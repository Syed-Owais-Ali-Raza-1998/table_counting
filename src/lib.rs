pub fn table (table_num: u32, table_range: u32)
{
    let table = table_num;
    let range = table_range;
    let mut initial = 1;
    let mut multiply;

    loop {
        multiply = table*initial;
        println!("{}x{}={}",table,initial,multiply);
        initial= initial+1;
        if initial>range {break;}
    }
}