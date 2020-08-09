trait RegraCalculo {
    fn calcula(&self, funcionario: &Funcionario) -> f64;
}


enum Cargo {
    Desenvolvedor(Box<dyn RegraCalculo>),
    Dba(Box<dyn RegraCalculo>),
    Testador(Box<dyn RegraCalculo>)
}


struct Funcionario {
    nome: String,
    salario: f64,
    cargo: Cargo,
}



#[derive(Copy, Clone)]
struct DezOuVintePorCento {} // shit!


impl RegraCalculo for DezOuVintePorCento {
    fn calcula(&self, funcionario: &Funcionario) -> f64 {
        if funcionario.salario > 3000f64 {
            funcionario.salario * 0.8
        } else {
            funcionario.salario * 0.9
        }
    }
}


#[derive(Copy, Clone)]
struct QuinzeOuVinteCincoPorcento {} //shit!


impl RegraCalculo for QuinzeOuVinteCincoPorcento {
    fn calcula(&self, funcionario: &Funcionario) -> f64 {
        if funcionario.salario < 2500f64 {
            funcionario.salario * 0.85
        } else {
            funcionario.salario * 0.75
        }
    }
}


fn calcula_salario(funcionario: &Funcionario) -> f64 {
    match &funcionario.cargo {
        Cargo::Desenvolvedor(regra_calculo) |
        Cargo::Dba(regra_calculo) |
        Cargo::Testador(regra_calculo) => regra_calculo.calcula(&funcionario)
    }
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn deve_calcular_salario_para_dev_com_salario_abaixo_do_limite() {
        let regra_dez_vinte = Box::new(DezOuVintePorCento {});

        let dev = Funcionario {
            nome: String::from("Allan"),
            salario: 1500f64,
            cargo: Cargo::Desenvolvedor(regra_dez_vinte)
        };

        let salario = calcula_salario(&dev);
        assert_eq!(1500f64 * 0.9f64, salario);
    }

    #[test]
    fn deve_calcular_salario_para_dev_com_salario_acima_do_limite() {
        let regra_dez_vinte = Box::new(DezOuVintePorCento {});

        let dev = Funcionario {
            nome: String::from("Allan"),
            salario: 4000f64,
            cargo: Cargo::Desenvolvedor(regra_dez_vinte)
        };

        let salario = calcula_salario(&dev);
        assert_eq!(4000f64 * 0.8f64, salario);
    }

    #[test]
    fn deve_calcular_salario_para_dba_com_salario_abaixo_do_limite() {
        let regra_quinze_vintecinco = Box::new(QuinzeOuVinteCincoPorcento{});

        let dba = Funcionario {
            nome: String::from("Allan"),
            salario: 500f64,
            cargo: Cargo::Dba(regra_quinze_vintecinco)
        };
        let salario = calcula_salario(&dba);
        assert_eq!(500f64 * 0.85, salario);
    }

    #[test]
    fn deve_calcular_salario_para_dba_com_salario_acima_do_limite() {
        let regra_quinze_vintecinco = Box::new(QuinzeOuVinteCincoPorcento{});

        let dba = Funcionario {
            nome: String::from("Allan"),
            salario: 4500f64,
            cargo: Cargo::Dba(regra_quinze_vintecinco)
        };
        let salario = calcula_salario(&dba);
        assert_eq!(4500f64 * 0.75, salario);
    }

    fn deve_calcular_salario_para_testador_com_salario_abaixo_do_limite() {
        let regra_quinze_vintecinco = Box::new(QuinzeOuVinteCincoPorcento{});

        let dba = Funcionario {
            nome: String::from("Allan"),
            salario: 500f64,
            cargo: Cargo::Testador(regra_quinze_vintecinco)
        };
        let salario = calcula_salario(&dba);
        assert_eq!(500f64 * 0.85, salario);
    }

    #[test]
    fn deve_calcular_salario_para_testador_com_salario_acima_do_limite() {
        let regra_quinze_vintecinco = Box::new(QuinzeOuVinteCincoPorcento{});

        let dba = Funcionario {
            nome: String::from("Allan"),
            salario: 4500f64,
            cargo: Cargo::Testador(regra_quinze_vintecinco)
        };
        let salario = calcula_salario(&dba);
        assert_eq!(4500f64 * 0.75, salario);
    }
}