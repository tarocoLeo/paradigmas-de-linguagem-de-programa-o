#### Paradigmas de Linguagem de Programação
# Projeto final

#### Instruções
1) Utilizar o mesmo princípio do código utilizado no comparativo (um carrinho de compra).

2) Processar vários carrinhos simultaneamente.

##### Escrever o código em:

C, RUST, Ruby e GO. 

##### Analisar as linguagens conforme:

Segurança, facilidade de aprendizado, velocidade de desenvolvimento, paradigma da linguagem.

#### Integrantes:
Isabelle Araujo, 60382-1
José Vitor, 60331-7
Leonardo Taroco, 60776-2
Rodrigo de Oliveira, 60253-1
Vitor Alexandre, 60913-7

#### Como executar

Para facilitar a vida de todos, utilizaremos Docker. Também iremos executar os códigos mostrando a velocidade de execução/compilação.

##### C
```bash
docker run -v $(pwd):/c -w/c -it gcc:latest bash
time cl c_cart.c
# esse aqui desiste, executa direto no https://replit.com/languages/c
# usa o docker só pra ver a velocidade de compilação
```
##### Rust
```bash
docker run -v $(pwd):/rust -w/rust -it rust:latest bash
time rustc -o main rust_cart.rs
time ./main
```

##### Ruby
```bash
docker run -v $(pwd):/ruby -w/ruby -it ruby:latest bash
time ruby ruby_cart.rb
```

##### Golang
```bash
docker run -v $(pwd):/go -w/go -it golang:latest bash
time go run go_cart.go
```