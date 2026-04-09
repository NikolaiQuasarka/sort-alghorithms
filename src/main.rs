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

        let (numbers, errors): (Vec<_>, Vec<_>) = numbers
            .split_whitespace()
            .enumerate()
            .map(|(i, n)| n.parse::<f64>().or(Err((i, n))))
            .partition(|n| n.is_ok());

        if !errors.is_empty() {
            let wrong_numbers = errors
                .iter()
                .map(|number| {
                    let (index, number) = number.expect_err("Значение прошло парсинг");

                    format!("\nПозиция: {index} - {number}")
                })
                .collect::<String>();

            eprintln!("Ошибка:{}\nВведите числа еще раз:", wrong_numbers);
            continue;
        };

        break numbers
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .expect("Значение не прошло парсинг");
    };

    numbers.fast_sort();

    let numbers = numbers
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("Отсортированные числа: {numbers}");
}
