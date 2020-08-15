use chrono::prelude::*;


struct Pedido {
    cliente: String,
    valor_total: f64,
    qtd_itens: u16
}


struct NotaFiscal {
    cliente: String,
    valor: f64,
    data: DateTime<Local>
}


trait AcaoAposGerarNota {
    fn executa(&self, nf: &NotaFiscal);
}


struct GeradorNF<'a> {
    acoes_pos_geracao: &'a Vec<&'a Box<dyn AcaoAposGerarNota + 'a>>
}


impl<'a> GeradorNF<'a>  {
    fn new(acoes_pos_geracao: &'a Vec<&'a Box<dyn AcaoAposGerarNota + 'a>>) -> Self {
        GeradorNF {
            acoes_pos_geracao
        }
    }

    fn gerar(self, pedido: &Pedido) -> NotaFiscal {
        let nf = NotaFiscal {
            cliente: pedido.cliente.clone(),
            data: Local::now(),
            valor: pedido.valor_total * 0.94
        };

        for acao in self.acoes_pos_geracao {
            acao.executa(&nf);
        }

        nf
    }
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct AcaoAposGerarNotaMock {
        executa_call_count: RefCell<u32>
    }

    impl AcaoAposGerarNotaMock {
        fn new() -> AcaoAposGerarNotaMock {
            AcaoAposGerarNotaMock {
                executa_call_count: RefCell::new(0)
            }
        }
    }

    impl AcaoAposGerarNota for &AcaoAposGerarNotaMock{
        fn executa(&self, nf: &NotaFiscal) {
            *self.executa_call_count.borrow_mut() += 1;
        }
    }


    #[test]
    fn deve_gerar_nf_com_impostos_descontados() {
        let acoes = vec![];
        let gerador_nf = GeradorNF::new(&acoes);

        let pedido = Pedido {
            cliente: String::from("Allan"),
            valor_total: 1000.0,
            qtd_itens: 1
        };

        let nf = gerador_nf.gerar(&pedido);

        assert_eq!(940f64, nf.valor);
    }

    #[test]
    fn deve_invocar_acoes_pos_geracao() {
        let acao_1 = AcaoAposGerarNotaMock::new();
        let acao_1_ref = Box::<dyn AcaoAposGerarNota>::from(box &acao_1);

        let acao_2 = AcaoAposGerarNotaMock::new();
        let acao_2_ref = Box::<dyn AcaoAposGerarNota>::from(box &acao_2);

        let acoes = vec![&acao_1_ref, &acao_2_ref];

        let gerador_nf = GeradorNF::new(&acoes);

        let pedido = Pedido {
            cliente: String::from("Allan"),
            valor_total: 1000.0,
            qtd_itens: 1
        };

        let nf = gerador_nf.gerar(&pedido);

        assert_eq!(1, *acao_1.executa_call_count.borrow());
        assert_eq!(1, *acao_2.executa_call_count.borrow());
    }
}
