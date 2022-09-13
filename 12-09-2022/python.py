# utilizando Python

cart = [
  {'id': 1, 'product': 'iPhone', 'price': 499},
  {'id': 2, 'product': 'Kindle', 'price': 179},
  {'id': 3, 'product': 'Macbook Pro', 'price': 1199}
]

def mapping(x):
  return x['price'] 

cartPrices = map(mapping, cart)
cartSum = sum(cartPrices)
print(cartSum)