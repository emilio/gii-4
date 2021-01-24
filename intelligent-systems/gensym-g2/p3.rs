#[derive(Copy, Clone, Debug)]
enum Action {
    Boletin,
    OfertaUltimaCompra,
    OfertaSnacks,
    OfertaRefrescos,
    OfertaFrescos,
    OfertaConservas,
    OfertaInfantiles,
    OfertaGourmet,
    OfertaRepetidos,
}

impl Action {
    fn to_symbol(self) -> &'static str {
        match self {
            Action::Boletin => "boletin",
            Action::OfertaUltimaCompra => "oferta_ultima_compra",
            Action::OfertaSnacks => "oferta_snacks",
            Action::OfertaRefrescos => "oferta_refrescos",
            Action::OfertaFrescos => "oferta_frescos",
            Action::OfertaConservas => "oferta_conservas",
            Action::OfertaInfantiles => "oferta_infantiles",
            Action::OfertaGourmet => "oferta_gourmet",
            Action::OfertaRepetidos => "oferta_repetidos",
        }
    }

    fn to_human(self) -> &'static str {
        match self {
            Action::Boletin => "Enviar boletin mensual",
            Action::OfertaUltimaCompra => "Enviar mail con ofertas sobre articulos de su ultima compra",
            Action::OfertaSnacks => "Hacer oferta de snacks, quesos, aceitunas",
            Action::OfertaRefrescos => "Hacer oferta de refrescos, zumos",
            Action::OfertaFrescos => "Hacer oferta de alimentos frescos",
            Action::OfertaConservas => "Hacer oferta de latas y conservas",
            Action::OfertaInfantiles => "Hacer oferta de productos infantiles",
            Action::OfertaGourmet => "Hacer ofertas de productos gourmet",
            Action::OfertaRepetidos => "Hacer oferta sobre articulos repetidos",
        }
    }
}

struct ConditionalNode {
    value: &'static str,
    node: DecisionTreeNode,
}

impl ConditionalNode {
    fn new(value: &'static str, node: DecisionTreeNode) -> Self {
        Self { value, node }
    }
}

enum DecisionTreeNode {
    Final(Action),
    Switch {
        variable: &'static str,
        conditions: Vec<ConditionalNode>,
    }
}

fn get_tree() -> DecisionTreeNode {
    DecisionTreeNode::Switch {
        variable: "compra_mes",
        conditions: vec![
            ConditionalNode::new("no", DecisionTreeNode::Final(Action::Boletin)),
            ConditionalNode::new("si", DecisionTreeNode::Switch {
                variable: "compras_ultimo_mes",
                conditions: vec![
                    ConditionalNode::new("una", DecisionTreeNode::Final(Action::OfertaUltimaCompra)),
                    ConditionalNode::new("dos_o_tres", DecisionTreeNode::Switch {
                        variable: "cerveza",
                        conditions: vec![
                            ConditionalNode::new("si", DecisionTreeNode::Final(Action::OfertaSnacks)),
                            ConditionalNode::new("no", DecisionTreeNode::Final(Action::OfertaRefrescos)),
                        ],
                    }),
                    ConditionalNode::new("mas_de_tres", DecisionTreeNode::Switch {
                        variable: "importe",
                        conditions: vec![
                            ConditionalNode::new("no", DecisionTreeNode::Switch {
                                variable: "limpieza",
                                conditions: vec![
                                    ConditionalNode::new("si", DecisionTreeNode::Final(Action::OfertaFrescos)),
                                    ConditionalNode::new("no", DecisionTreeNode::Final(Action::OfertaConservas)),
                                ],
                            }),
                            ConditionalNode::new("si", DecisionTreeNode::Switch {
                                variable: "repeticion",
                                conditions: vec![
                                    ConditionalNode::new("si", DecisionTreeNode::Final(Action::OfertaRepetidos)),
                                    ConditionalNode::new("no", DecisionTreeNode::Switch {
                                        variable: "pannales",
                                        conditions: vec![
                                            ConditionalNode::new("si", DecisionTreeNode::Final(Action::OfertaInfantiles)),
                                            ConditionalNode::new("no", DecisionTreeNode::Final(Action::OfertaGourmet)),

                                        ]
                                    }),
                                ],
                            }),
                        ],
                    }),
                ],
            }),
        ],
    }
}

struct RuleExtractor {
    conditions_stack: Vec<String>,
    variables: Vec<&'static str>,
    rules: Vec<String>,
    use_metaknowledge: bool,
}

impl RuleExtractor {
    fn extract(use_metaknowledge: bool, tree: &DecisionTreeNode) -> Vec<String> {
        let mut extractor = RuleExtractor {
            conditions_stack: vec![],
            variables: vec![],
            rules: vec![],
            use_metaknowledge,
        };

        extractor.visit(tree);

        {
            let mut initial_rule = String::new();
            initial_rule.push_str("initially in order show inicio");
            for variable in &extractor.variables {
                initial_rule.push_str(" and conclude that the ");
                initial_rule.push_str(variable);
                initial_rule.push_str(" of paco is unknown");
            }
            initial_rule.push_str(" and conclude that the accion of paco is unknown");
            extractor.rules.push(initial_rule);
        }

        extractor.rules
    }

    fn current_condition(&self) -> String {
        if self.use_metaknowledge {
            if let Some(last_condition) = self.conditions_stack.last() {
                return last_condition.clone();
            }
            return String::new();
        }

        let mut result = String::new();
        for (i, condition) in self.conditions_stack.iter().enumerate() {
            if i != 0 {
                result.push_str(" and ");
            }
            result.push_str(condition);
        }
        result
    }

    fn visit(&mut self, tree: &DecisionTreeNode) {
        match *tree {
            DecisionTreeNode::Final(action) => {
                let mut rule = String::new();
                rule.push_str("for any cliente V if ");
                rule.push_str(&self.current_condition());
                rule.push_str(" then conclude that the accion of V is ");
                rule.push_str(action.to_symbol());
                self.rules.push(rule);

                rule = String::new();
                rule.push_str("whenever the accion of any cliente V receives a value and when the accion of V is ");
                rule.push_str(action.to_symbol());
                rule.push_str(" then inform the operator on the subworkspace of paco that ");
                rule.push_str("\"La accion para paco es ");
                rule.push_str(action.to_human());
                rule.push_str("\" and show the subworkspace of paco and pause knowledge-base");
                self.rules.push(rule);
            }
            DecisionTreeNode::Switch { variable, ref conditions } => {
                self.variables.push(variable);

                let mut rule = String::new();
                rule.push_str("for any cliente V if the ");
                rule.push_str(variable);
                rule.push_str(" of V is unknown");
                if !self.conditions_stack.is_empty() {
                    rule.push_str(" and ");
                    rule.push_str(&self.current_condition());
                }
                rule.push_str(" then show pregunta_");
                rule.push_str(variable);
                rule.push_str(" at the center of the screen");
                self.rules.push(rule);

                rule = String::new();
                rule.push_str("whenever var_");
                rule.push_str(variable);
                rule.push_str(" receives a value then conclude that the ");
                rule.push_str(variable);
                rule.push_str(" of paco = the value of var_");
                rule.push_str(variable);
                self.rules.push(rule);

                for condition in conditions {
                    self.conditions_stack.push(format!("the {} of V is {}", variable, condition.value));
                    self.visit(&condition.node);
                    self.conditions_stack.pop();
                }
            }
        }
    }
}

fn main() {
    let tree = get_tree();
    let use_metaknowledge = std::env::args().len() > 1;
    for rule in RuleExtractor::extract(use_metaknowledge, &tree) {
        println!("{}", rule);
    }
}
