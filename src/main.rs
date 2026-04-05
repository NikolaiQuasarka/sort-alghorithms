use std::io;

use sorts_alghorithms::QuickSort;

fn main() {
    loop {
        app();
    }
}

fn app() {
    println!("Введите числа для сортировки:");
    let mut numbers = loop {
        let mut numbers = String::new();

        if let Err(err) = io::stdin().read_line(&mut numbers) {
            eprint!("Ошибка: {}", err);
            continue;
        }

        let numbers = match numbers
            .split_whitespace()
            .map(|n| n.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()
        {
            Ok(numbers) => numbers,
            Err(err) => {
                eprintln!("Ошибка: {}\nВведите числа еще раз:", err);
                continue;
            }
        };

        break numbers;
    };

    numbers.as_mut_slice().fast_sort(&|n1, n2| n1 < n2);

    let numbers = numbers
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("Отсортированные числа: {numbers}");
}
