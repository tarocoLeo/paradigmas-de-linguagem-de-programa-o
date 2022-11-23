package main
import ("fmt"; "sync" )

type ShopCart struct {
	name string
	price int
}

func total_price(shop_cart[]ShopCart, cart_size int){
	sum:=0;

	for item := 0; item < cart_size; item++ {
		sum += shop_cart[item].price
	}

	fmt.Println(sum);
	wg.Done()
}

var wg sync.WaitGroup

func main(){
	shop_cart1 := []ShopCart{
		ShopCart {name: "item 1", price: 100},
		ShopCart {name: "item 2", price: 300},
		ShopCart {name: "item 3", price: 150},
	}

	shop_cart2 := []ShopCart{
		ShopCart {name: "item 4", price: 200},
		ShopCart {name: "item 5", price: 250},
		ShopCart {name: "item 6", price: 110},
	}

	shop_cart3 := []ShopCart{
		ShopCart {name: "item 7", price: 130},
		ShopCart {name: "item 8", price: 700},
		ShopCart {name: "item 9", price: 70},
	}

	wg.Add(3)
	go total_price(shop_cart1, 3)
	go total_price(shop_cart2, 3)
	go total_price(shop_cart3, 3)
	wg.Wait()
}
