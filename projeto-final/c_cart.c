#include <stdio.h>
#include <pthread.h>

typedef struct{
  char name[10];
  int price;
}shop_carts;

void total_price(shop_carts cart[]){
  int item, sum=0;
  for (item=0; item<3; item++){ sum+=cart[item].price; }
  printf("%d\n", sum);
}

int main(){
  pthread_t thread1, thread2, thread3;

  int sum;
  shop_carts shop_cart1[3] = {
    {"item 1", 100,}, {"item 2", 300}, {"item 3", 150}
  };

  shop_carts shop_cart2[3] = {
    {"item 4", 200,}, {"item 5", 250}, {"item 6", 110}
  };

  shop_carts shop_cart3[3] = {
    {"item 7", 130,}, {"item 8", 700}, {"item 9", 70}
  };

  pthread_create(&thread1, NULL, (void *)total_price, shop_cart1);
  pthread_create(&thread2, NULL, (void *)total_price, shop_cart2);
  pthread_create(&thread3, NULL, (void *)total_price, shop_cart3);
  pthread_join(thread1, NULL);
  pthread_join(thread2, NULL);
  pthread_join(thread3, NULL);

  return 0;
}