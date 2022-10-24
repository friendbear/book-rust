struct Item(String, i64);

fn main() {
    let banana = Item("バナナ".to_string(), 300);
    let apple = Item("リンゴ".to_string(), 200);
    let mango = Item("マンゴ".to_string(), 500);

    let items = vec![banana, apple, mango];

    let total = sum_items(&items);
    println!("合計: {}", total);
}

fn sum_items(items: &Vec<Item>) -> i64 {
    let mut total = 0;
    for i in items {
        total += i.1;
    }
    total
}
