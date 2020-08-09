use std::vec::Vec;


#[derive(Clone)]
struct Item {
    descricao: String,
    quantidade: u16,
    valor_unitario: f64,
}


impl Item {
    fn valor_total(&self) -> f64 {
        self.valor_unitario * self.quantidade as f64
    }
}


struct Carrinho {
    itens: Vec<Item>
}


impl Carrinho {
    fn new() -> Self {
        Carrinho {
            itens: vec![]
        }
    }

    fn adiciona(&mut self, item: Item) {
        self.itens.push(item);
    }

    fn maior_valor(&self) -> f64 {
        self.itens
        .iter()
        .map(|item| item.valor_total())
        .fold(Some(0f64), |ac, x| {
            let valor_atual = ac.unwrap();
            if valor_atual < x {
                Some(x)
            } else {
                ac
            }
        }).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    struct CarrinhoBuilder {
        carrinho: Carrinho
    }


    impl CarrinhoBuilder {
        fn new() -> Self {
            CarrinhoBuilder {
                carrinho: Carrinho::new()
            }
        }

        fn com_items(mut self, valores: Vec<(u16, f64)>) -> Self {

            for (quantidade, valor_unitario) in valores.iter() {
                self.carrinho.adiciona(
                    Item {
                        descricao: String::from("TDD - Book"),
                        valor_unitario: *valor_unitario,
                        quantidade: *quantidade
                    }
                );
            }
            self
        }

        fn criar(self) -> Carrinho {
            self.carrinho
        }
    }

    #[test]
    fn deve_adicionar_um_item_no_carrinho() {
        let item = Item {
            descricao: String::from("TDD - Book"),
            quantidade: 3,
            valor_unitario: 35.90f64
        };

        let mut carrinho = Carrinho::new();
        carrinho.adiciona(item.clone());

        assert_eq!(1, carrinho.itens.len());
        assert_eq!(item.descricao, carrinho.itens[0].descricao);
        assert_eq!(item.quantidade, carrinho.itens[0].quantidade);
        assert_eq!(item.valor_unitario, carrinho.itens[0].valor_unitario);
    }

    #[test]
    fn deve_retornar_zero_se_carrinho_vazio() {
        let carrinho = CarrinhoBuilder::new().criar();
        assert_eq!(0.0f64, carrinho.maior_valor());
    }

    #[test]
    fn deve_retornar_valor_item_se_carrinho_1_elemento() {
        let carrinho = CarrinhoBuilder::new()
            .com_items(vec![(2, 150f64)])
            .criar();
        assert_eq!(300f64, carrinho.maior_valor());
    }

    #[test]
    fn deve_retornar_maior_carrinho_com_muitos_elementos() {
        let carrinho = CarrinhoBuilder::new()
            .com_items(vec![(1,300.01f64), (2, 150f64), (2, 149f64)])
            .criar();
        assert_eq!(300.01f64, carrinho.maior_valor())
    }
}
