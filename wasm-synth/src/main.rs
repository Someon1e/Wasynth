use parity_wasm::{deserialize_file, elements::Module};

use codegen_luajit::gen::Generator as LuaJIT;
use codegen_luau::gen::Generator as Luau;
use wasm_ast::writer::Transpiler;

fn parse_module(name: &str) -> Module {
	let wasm = deserialize_file(name).expect("Failed to parse Wasm file");

	wasm.parse_names().unwrap_or_else(|v| v.1)
}

fn run_translator<'a, T: Transpiler<'a>>(wasm: &'a Module) {
	let module = T::new(wasm);
	let output = std::io::stdout();

	module
		.transpile(&mut output.lock())
		.expect("Failed to transpile");
}

fn run_runtime<'a, T: Transpiler<'a>>() {
	let output = std::io::stdout();

	T::runtime(&mut output.lock()).expect("Failed to fetch runtime");
}

fn do_translate(name: &str, file: &str) {
	let wasm = &parse_module(file);

	match name.to_lowercase().as_str() {
		"luajit" => run_translator::<LuaJIT>(wasm),
		"luau" => run_translator::<Luau>(wasm),
		_ => panic!("Bad language: {}", name),
	}
}

fn do_runtime(name: &str) {
	match name.to_lowercase().as_str() {
		"luajit" => run_runtime::<LuaJIT>(),
		"luau" => run_runtime::<Luau>(),
		_ => panic!("Bad runtime: {}", name),
	}
}

fn do_help() {
	println!("usage: program translate <lang> <file>");
	println!("  or:  program runtime <lang>");
	println!("  or:  program help");
}

fn main() {
	let mut args = std::env::args().skip(1);

	match args.next().as_deref().unwrap_or("help") {
		"help" => do_help(),
		"runtime" => {
			let lang = args.next().expect("No runtime specified");

			do_runtime(&lang);
		}
		"translate" => {
			let lang = args.next().expect("No language specified");
			let file = args.next().expect("No file specified");

			do_translate(&lang, &file);
		}
		bad => {
			eprintln!("Bad action `{}`; try `help`", bad);
		}
	}
}
