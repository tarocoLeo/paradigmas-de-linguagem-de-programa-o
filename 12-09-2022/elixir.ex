# utilizando Elixir

cart = [
  %{id: 1, product: "IPhone", price: 499},
  %{id: 2, product: "Kindle", price: 179},
  %{id: 3, product: "Macbook Pro", price: 1199}
]

total =
  cart
  |> Enum.map(fn %{price: price} -> price end)
  |> Enum.sum()

IO.inspect total