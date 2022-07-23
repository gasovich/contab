// Создаем программу для консольной работы с простыми электронными таблицами
// Как часть офисного пакета с консольным интерфейсом

use std::io;
use std::io::Write;

fn main() {
	let mut buffer = String::new();
	let mut k: f64;
	
	struct Cell {
		row: usize,
		col: usize,
		num_value: Option<f64>,
		txt_value: Option<String>,
	}
	
	let mut cursor = Cell {
		row: 0,
		col: 0,
		num_value: None,
		txt_value: None,
	};

	let mut v: Vec<Cell> = Vec::new();
	let mut idx: usize = 0;
	
	loop { // Главный цикл программы
		// Готовим промпт
		let prompt = format!("[{}][{}]--> ", cursor.row, cursor.col);
		
		// Печатаем промпт
		print!("{}", prompt);
		match io::stdout().flush() {
			Ok(_) => {},
			Err(e) => println!("Ошибка ввода - {}", e),
		}
		
		// Читаем строку
		match io::stdin().read_line(&mut buffer) {
			Ok(_) => {},
			Err(e) => println!("Ошибка ввода - {}", e),
		}
		
		// Пытаемся преобразовать строку в число, получаем данные в формате Result
		let mut try_num = buffer.trim().parse();
		
		// Проверяем, чем оказался результат преобразования
		match try_num {
			Ok(_) => { // Либо это целое число
				k = try_num.unwrap(); // Тогда извлекаем из Result значение
				println!("Number: {}", k);
				let next = Cell {// Сохраняем число в буферной структуре
					row: 0,
					col: cursor.col,
					num_value: Some(k),
					txt_value: None,
					};
				v.push(next);// Отправляем числовое значение в массив, содержащий таблицу
			},
			Err(_) => {// В случае ошибки преобразования обрабатываем исходную строку
				let i: i32 = 0;
			    let mut symbol: char = ' ';
			    for c in buffer.chars() {// Похоже, это единственный способ получить символ из строки UTF8
					if i == 0 {// Берем нулевой символ из буфера 
						symbol = c;
						break;
					}
				};
				if symbol == '!' {// Если введене команда ( !<команда> )
					println!("Command: {}", buffer.trim());
					if buffer.trim() == "!stop".to_string() {// Команда выхода из главного цикла
						break;
					}
				} else {// Иначе строка интерпретируется, как значение ячейки 
					println!("String: {}", buffer.trim());
					let next = Cell {
						row: 0,
						col: cursor.col,
						num_value: None,
						txt_value: Some(buffer),
					};
					v.push(next);
				};
			}
		}
		
		buffer = "".to_string();
		cursor.col += 1;
	};
	
	for d in &v {
		let x: f64;
		let y = String::new();
		match d.num_value {
			Some(x) => println!("v[{}][{}] = {:?}", d.row, d.col, d.num_value.unwrap()),
			None => {},
		};
		
		match d.txt_value {
			Some(_) => {
				println!("v[{}][{}] = {}", d.row, d.col, d.txt_value.as_ref().unwrap().trim());
			},
			None => {},
		};
	};
}
