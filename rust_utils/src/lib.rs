use chrono::prelude::*;
use polars::prelude::*;

fn main() {
    let df: DataFrame = df!(
        "name" => ["Alice", "Bob"],
        "age" => [25, 30],
    )
    .unwrap();

    println!("{}", df);
}
