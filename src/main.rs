use std::{fmt::Error, ops::Rem};

fn get_id(ch: char, alphabet: String) -> i32{
// получаем код символа
    let N = alphabet.len();
    for i in 0..N {
        if ch == alphabet.chars().nth(i).unwrap() {
            return i as i32;
        }
    }
    return -1;
}

// шифр Цезаря
fn cesar(str_in: String, alphabet: String, key: i32) -> Option<String> {
    let N = alphabet.len(); // мощность алфавита
    let mut str_out = String::new();
    for ch in str_in.chars() {
        let id = get_id(ch, alphabet.clone());
        if id == -1 {
            
            print!("Символа {} не существует в алфавите!!!", ch);
            return None;
        }
        else {
            // шифруем
            let new_id = (id + key).rem_euclid(N as i32); // по модулю 
            let new_ch = alphabet.chars().nth(new_id as usize).unwrap();
            str_out.push(new_ch);
        }
    }
    return Some(str_out);
}

// дешифратор Цезаря
fn cesar_dec(str_in: String, alphabet: String, key: i32) -> Option<String> {
    let N = alphabet.len(); // мощность алфавита
    let mut str_out = String::new();
    for ch in str_in.chars() {
        let id = get_id(ch, alphabet.clone());
        if id == -1 {
            print!("Символа {} не существует в алфавите!!!", ch);
            return None;
        }
        else {
            // шифруем
            let new_id = (id - key).rem_euclid(N as i32); // по модулю 
            let new_ch = alphabet.chars().nth(new_id as usize).unwrap();
            str_out.push(new_ch);
        }
    }
    return Some(str_out);
}

// =================================
// шифр Виженера
fn vigenere(str_in: String, alphabet: String, key: String) -> Option<String> {
    // проверка длин
    if str_in.len() != key.len() {
        println!("Длина шифруемого слова и ключа должна быть одинаковой");
        return None;
    }
    let n: usize = alphabet.len();
    let mut str_out = String::new();
    // проходимся по входной строке
    for (i, ch) in str_in.chars().enumerate() {
        // находим символ строки в алфавите
        let id = get_id(ch, alphabet.clone()); 
        if id == -1 {
            return None; // Символ не найден в алфавите
        }
        // настрой символ ключа в алфавите
        let id_key = get_id(key.chars().nth(i).unwrap(), alphabet.clone());
        let new_id = (id + id_key).rem_euclid(n as i32);
        // зашифрованный символ = пересечение символа строки с символом ключа
        let new_ch = alphabet.chars().nth(new_id as usize).unwrap();
        str_out.push(new_ch);
    }
    
    Some(str_out)
}

// дешифратор 
fn vigenere_dec(str_in: String, alphabet: String, key: String) -> Option<String> {
    // Проверка длин
    if str_in.len() != key.len() {
        println!("Длина зашифрованного слова и ключа должна быть одинаковой");
        return None;
    }
    
    let n: usize = alphabet.len();
    let mut table: Vec<Vec<char>> = vec![vec![]; n];
    
    // Заполнение таблицы алфавитом
    for i in 0..n {
        for j in 0..n {
            table[i].push(alphabet.chars().nth((i + j) % n).unwrap());
        }
    }
    
    let mut str_out = String::new();
    
    // Дешифрование
    for (i, ch) in str_in.chars().enumerate() {
        // Сначала ищем в алфавите символ, который соответствует символу во входной строке
        let id = get_id(ch, alphabet.clone()); 
        if id == -1 {
            return None; // Символ не найден в алфавите
        } else {
            // Находим символ ключа, соответствующий текущему символу строки
            let key_id = get_id(key.chars().nth(i % key.len()).unwrap(), alphabet.clone());
            // Вычисляем позицию символа в строке алфавита
            let mut new_ch_id = 0;
            while table[key_id as usize][new_ch_id] != ch {
                new_ch_id += 1;
            }
            str_out.push(alphabet.chars().nth(new_ch_id).unwrap());
        }
    }
    
    Some(str_out)
}

// print vigenere-table
fn print_vigenere_table(alphabet: String) {
    let n: usize = alphabet.len();
    let mut table: Vec<Vec<char>> = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            table[i].push(alphabet.chars().nth((i + j) % n).unwrap());
        }
    }
    for i in 0..n {
        for j in 0..n {
            print!("{} ", table[i][j]);
        }
        println!();
    }
}
// =================================
// для отрисовки таблицы
fn print_scytale_table(matrix: &[Vec<char>]) {
    for row in matrix {
        for &ch in row {
            print!("{} ", ch);
        }
        println!();
    }
}
// шифр скитала
fn scytale(str_in: String, rows: usize) -> String {
    let str_len = str_in.len();
    let cols = (str_len as f64 / rows as f64).ceil() as usize; // Вычисляем количество столбцов
    let mut matrix: Vec<Vec<char>> = vec![vec![' '; cols]; rows]; // Создаем пустую матрицу
    let mut str_idx = 0; // построчный счетчик

    // Заполняем матрицу по порядку символами из входной строки
    // (пустые ячейки заполнены пустотой)
    for i in 0..rows {
        for j in 0..cols {
            if str_idx < str_len {
                matrix[i][j] = str_in.chars().nth(str_idx).unwrap();
                str_idx += 1;
            }
        }
    }

    // Транспонируем: Считываем символы из матрицы в порядке сверху вниз, слева направо
    let mut result = String::new();
    for j in 0..cols {
        for i in 0..rows {
            result.push(matrix[i][j]);
        }
    }

    // отрисовка таблицы
    print_scytale_table(&matrix);

    result
}

// дешифратор шифра скитала
fn scytale_dec(str_in: String, rows: usize) -> String {
    let str_len = str_in.len();
    let cols = (str_len as f64 / rows as f64).ceil() as usize; // Вычисляем количество столбцов
    let mut matrix: Vec<Vec<char>> = vec![vec![' '; cols]; rows]; // Создаем пустую матрицу
    let mut str_idx = 0; // Индекс для прохода по входной строке

    // Заполняем матрицу символами из входной строки
    for j in 0..cols {
        for i in 0..rows {
            if str_idx < str_len {
                matrix[i][j] = str_in.chars().nth(str_idx).unwrap();
                str_idx += 1;
            }
        }
    }
    // Восстанавливаем: Считываем символы из матрицы по порядку слева направо, сверху вниз
    let mut result = String::new();
    for i in 0..rows {
        for j in 0..cols {
            result.push(matrix[i][j]);
        }
    }

    result
}

fn main() {
    // шапка
    println!("=================================");
    println!("Шифр Цезаря");
    let alphabet = String::from("abcdefghijklmnopqrstuvwxyz");
    let str_in = String::from("hello");
    println!("Алфавит: {}", alphabet);
    println!("Начальная строка: {}", str_in);
    let key = 7;
    println!("Ключ: {}", key);
    // test
    let str_out = cesar(str_in, alphabet, key).unwrap();
    println!("Зашифрованная: {}", str_out);
    println!("Расшифрованная: {}", cesar_dec(str_out, String::from("abcdefghijklmnopqrstuvwxyz"), key).unwrap());

    println!("=================================");
    println!("Шифр Виженера");
    print_vigenere_table(String::from("abcdefghijklmnopqrstuvwxyz"));
    let str_in = String::from("hello");
    println!("Начальная строка: {}", str_in);
    let key = String::from("helpm");
    println!("Ключ: {}", key);
    // test
    let str_out = vigenere(str_in, String::from("abcdefghijklmnopqrstuvwxyz"), key).unwrap();
    println!("Зашифрованная: {}", str_out);
    println!("Расшифрованная: {}", vigenere_dec(str_out, String::from("abcdefghijklmnopqrstuvwxyz"), String::from("helpm")).unwrap());

    println!("=================================");
    println!("Шифр Скитала");
    let str_in = String::from("hello");
    println!("Начальная строка: {}", str_in);
    let rows = 3;
    println!("Количество строк: {}", rows);
    // test
    let str_out = scytale(str_in, rows);
    println!("Зашифрованная: {}", str_out);
    println!("Расшифрованная: {}", scytale_dec(str_out, rows));

}
