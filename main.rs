const PI:f32 = 3.14;
static mut VARIAVEL_GLOBAL:u8 = 1;

fn soma(a:i32, b:i32) -> i32 {
	println!("{} + {} = {}", a, b, a + b);
	a + b
}

fn escopo() {
	println!("PI = {}", PI);
	unsafe{
		println!("Varivael global = {}", VARIAVEL_GLOBAL);
	}

	let inteiro = 300;
	println!("variavel = {}, tamanho = {} bytes", inteiro, std::mem::size_of_val(&inteiro));

	let decimal:f32 = 2.5;
	println!("decimal = {}, tamanho = {} bytes", decimal, std::mem::size_of_val(&decimal));
	
	let booleano = false;
	println!("booleana = {}, tamanho = {} bytes", booleano, std::mem::size_of_val(&booleano));

	let letra:char = 'C';
	println!("catacter = {}, tamanho = {} bytes", letra, std::mem::size_of_val(&letra));
}

fn sombra() {
	let a = 123;

	{
		let b = 456;
		println!("Dentro, b = {}", b);

		let a = 777;
		println!("Dentro, a = {}", a);
	}


	println!("Fora, a = {}", a);
}

fn condicionais() {
	println!("Soma = {}", soma(2,2));

	let idade: u8 = 20;
	let responsavel_autorizou = true;
	let eh_maior = idade >= 18;
	let eh_maior_16 = idade >= 16;

	if eh_maior {
		println!("Pode entrar na balada");
	} else if eh_maior_16 && responsavel_autorizou {
		println!("Pode entrar com assinatura do resposável.")
	} else {
		println!("Não pode entrar na balada")
	}

	let condicao = if eh_maior { "maior "} else {" menor "};

	println!("Essa pessoa é {} de idade", condicao);

	let linguagem = "PHP";
	let proposito = match linguagem {
		"PHP" => "Web",
		"Kotlin" => "Android",
		"Python" => "Data Science",
		_ => "Desconhecido"
	};

	println!("O proposito de {} é {}", linguagem, proposito);
}

fn repeticoes() {
	let multiplicador:u8 = 5;
	let mut contador:u8 = 0;
	while contador < 10 {
		contador += 1;

		if contador == 5 {
			continue;
		}

		println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
	} 
	
	contador = 0;
	loop {
		contador += 1;
		println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);

		if contador == 10 { 
			break;
		}
	}

	for i in 1..11 {
		println!("{} x {} = {}", multiplicador, i, multiplicador * i);
	}
}

fn ownership() {
	let mut uma_string = String::from("Enrico");
	rouba(&mut uma_string);

	println!("{}", uma_string);

}

fn rouba(string: &mut String) {
	string.push_str("Papsch");
	println!("{}", string);
}

fn pattern_matching() {
	for x in 1..=20 {
		println!("{}: {}", x, match x {
			1 => "Pouco",
			2 | 3 => "Um poquinho",
			4..=10 => "Um bocado",
			_ if x % 2 == 0 => "Uma boa quantidade",
			_ => "Muito"
		});
	}
}

fn erros() {
	match resultado() {
		Ok(s) => println!("String de sucesso: {}", s),
		Err(numero) => println!("Código de erro = {}", numero)
	};
}

fn resultado() -> Result<String, u8>
{
	Ok(String::from("Deu tudo certo"))
	
}

fn main() {
	escopo();
	sombra();
	repeticoes();
	condicionais();
	ownership();
	pattern_matching();
	erros();
}