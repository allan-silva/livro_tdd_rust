
#[allow(non_snake_case)]

use std::collections::HashMap;


fn main() {
    println!("Hello, world!");
}


struct ConversorNumeroRomano {
    symbols: HashMap<char, i32>
}


impl ConversorNumeroRomano {
    fn new() -> Self {
        let mut symbols = HashMap::new();
        symbols.insert('I', 1);
        symbols.insert('V', 5);
        symbols.insert('X', 10);
        symbols.insert('L', 50);
        symbols.insert('C', 100);
        symbols.insert('D', 500);
        symbols.insert('M', 1000);
        ConversorNumeroRomano {
            symbols
        }
    }

    fn converte(&self, numero_romano : &str) -> Result<i32, String> {
        let mut valor = 0;

        for c in numero_romano.chars() {
            valor += *self.symbols.get(&c).ok_or(format!("Caracter invalido: {}", c))?;
        }

        Ok(valor)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use crate::ConversorNumeroRomano;

    #[test]
    fn deve_entender_I() {
        let romano = ConversorNumeroRomano::new();
        let numero = romano.converte("I");
        assert_eq!(Result::Ok(1), numero);
    }

    #[test]
    fn deve_entender_V() {
        let romano = ConversorNumeroRomano::new();
        let numero = romano.converte("V");
        assert_eq!(Result::Ok(5), numero);
    }

    #[test]
    fn deve_entender_dois_simbolos_II() {
        let romano = ConversorNumeroRomano::new();
        let numero = romano.converte("II");
        assert_eq!(Result::Ok(2), numero);
    }
}