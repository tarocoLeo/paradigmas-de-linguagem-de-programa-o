use std::thread;

pub struct ShopCart {
  _name: &'static str,
  price: i32,
}

fn total_price(a: &[ShopCart]) -> () {
  let sum:i32 = a.iter().map(
    |s| s.price
  ).sum();
  println!("{:?}", sum);
}

fn main() {
  const SHOPCART1: [ShopCart; 3] = [
    ShopCart {_name: "item 1", price: 100},
    ShopCart {_name: "item 2", price: 300},
    ShopCart {_name: "item 3", price: 150},
  ];

  const SHOPCART2: [ShopCart; 3] = [
    ShopCart {_name: "item 4", price: 200},
    ShopCart {_name: "item 5", price: 250},
    ShopCart {_name: "item 6", price: 110},
  ];

  const SHOPCART3: [ShopCart; 3] = [
    ShopCart {_name: "item 7", price: 130},
    ShopCart {_name: "item 8", price: 700},
    ShopCart {_name: "item 9", price: 70},
  ];

  let t1 = thread::spawn(|| { total_price(&SHOPCART1); });
  let t2 = thread::spawn(|| { total_price(&SHOPCART2); });
  let t3 = thread::spawn(|| { total_price(&SHOPCART3); });

  t1.join().unwrap();
  t2.join().unwrap();
  t3.join().unwrap();
}