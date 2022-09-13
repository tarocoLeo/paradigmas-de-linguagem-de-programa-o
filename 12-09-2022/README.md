## Desafio

Reescrever o trecho de código abaixo em outras 3 linguagens de programação utilizando o paradigma funcional:

```js
// shopping cart
const cart = [
  {id: 1, product: 'iPhone', price: 499},
  {id: 2, product: 'Kindle', price: 179},
  {id: 3, product: 'Macbook Pro', price: 1199}
]

// get prices from shopping cart and sum them
// using function composition
const totalCart = R.compose(
    R.sum,
    R.map(item => item.price),
);

console.log(totalCart(cart)); // 1877
