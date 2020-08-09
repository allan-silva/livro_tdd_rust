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


trait SAPIntegration {
    fn send(&self, nota_fiscal: &NotaFiscal);
}


trait DB {
    fn persiste(&self, nota_fiscal: &NotaFiscal);
}


struct GeradorNF<'a> {
    sap_client: &'a dyn SAPIntegration,
    db: &'a dyn DB
}


impl<'a> GeradorNF<'a> {
    fn new(db: &'a dyn DB, sap_client: &'a dyn SAPIntegration) -> Self {
        GeradorNF {
            sap_client,
            db
        }
    }

    fn gerar(&mut self, pedido: &Pedido) -> NotaFiscal {
        let nf = NotaFiscal {
            cliente: pedido.cliente.clone(),
            data: Local::now(),
            valor: pedido.valor_total * 0.94
        };

        self.db.persiste(&nf);

        nf
    }
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;


    struct SAPTestClient {}


    impl SAPIntegration for SAPTestClient {
        
        fn send(&self, notafiscal : &NotaFiscal) {}
    }


    struct DBTest {}


    impl DB for DBTest {
        fn persiste(&self, nota_fiscal: &NotaFiscal) {
        }
    }


    #[test]
    fn deve_gerar_nf_com_impostos_descontados() {
        let sap_test_client = SAPTestClient{};
        let db_test = DBTest{};

        let mut gerador_nf = GeradorNF::new(&db_test, &sap_test_client);

        let pedido = Pedido {
            cliente: String::from("Allan"),
            valor_total: 1000.0,
            qtd_itens: 1
        };

        let nf = gerador_nf.gerar(&pedido);

        assert_eq!(940f64, nf.valor);
    }
}
