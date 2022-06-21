# Пример использования библиотеки <a href="https://github.com/DimartX/hadamard-secret-sharing"> hadamard-secret-sharing </a>

## Скачивание репозитория
```bash
git clone git@github.com:DimartX/hadamard_sss_example.git
```
## Сборка и запуск
```bash
cd hadamard_sss_example
cargo run
```
## Пояснение примера `simple_example`
Добыча матрицы отдаётся на откуп пользователю библиотеки. Предполагается хранение матрицы у дилера.

Считываем матрицу Адамара (взята с http://neilsloane.com/hadamard/) из файла.

```rust
let mtx = read_matrix("../matrices/had8.txt").expect("can't read matrix");
```

Создаём экземляр структуры схемы разделения секрета.

```rust
let scheme = HadamardSSS::from(&mtx).expect("can't create scheme");
```

Создаём секрет типа u32 и разделяем его на доли, количество которых на одну меньше размерности используемой матрицы.

```rust
let secret: u32 = 314159265;
let parts = scheme.share(secret).expect("can't share that secret");
```

Демонстрируем результат при попытке восстановить секрет по числу долей, меньшему порогового значения.

```rust
let parts_rec = vec![parts[1], parts[3], parts[6]];
let res_secret = scheme.reconstruct(parts_rec);
match res_secret {
    Ok(res) => println!("Result is {}", res),
    Err(err_msg) => println!("can't reconstruct secret: {}", err_msg),
}
```

Также демонстрируем успешное восстановление секрета.

```rust
let parts_rec = vec![parts[1], parts[3], parts[4], parts[5], parts[6]];
let res_secret = scheme.reconstruct(parts_rec);
match res_secret {
    Ok(res) => println!("Result is {}", res),
    Err(err_msg) => println!("can't reconstruct secret: {}", err_msg),
}
```

Результат выполнения примера `simple_example`:

```
Matrix was read:
[[1, 1, 1, 1, 1, 1, 1, 1],
 [1, -1, 1, -1, 1, -1, 1, -1],
 [1, 1, -1, -1, 1, 1, -1, -1],
 [1, -1, -1, 1, 1, -1, -1, 1],
 [1, 1, 1, 1, -1, -1, -1, -1],
 [1, -1, 1, -1, -1, 1, -1, 1],
 [1, 1, -1, -1, -1, -1, 1, 1],
 [1, -1, -1, 1, -1, 1, 1, -1]]
The initial secret is 314159265
Parts:
(0, 1480438321)
(1, 446694049)
(2, 114636929)
(3, 415081689)
(4, 1203303429)
(5, 2973513135)
(6, 2746864041)
Trying to reconstruct by 3 random values:
3 is less than threshold 5 parties
can't reconstruct secret: less than threshold parties
Trying to reconstruct by 5 random values:
Result is 314159265
```
