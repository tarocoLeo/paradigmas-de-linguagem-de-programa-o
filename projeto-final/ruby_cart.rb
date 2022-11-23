all_carts = [
  [
    {"name": "item 1", "price": 100},
    {"name": "item 2", "price": 300},
    {"name": "item 3", "price": 150}
  ],
  [
    {"name": "item 4", "price": 200},
    {"name": "item 5", "price": 250},
    {"name": "item 6", "price": 110}
  ],
  [
    {"name": "item 7", "price": 130},
    {"name": "item 8", "price": 700},
    {"name": "item 9", "price": 70}
  ]
]

def total_price(*shop_cart)
   Thread.new{puts shop_cart.map {
    |item| item[:price]
  }.reduce(:+)}
end

all_carts.each do |shop_cart|
  total_price(*shop_cart).join()
end
