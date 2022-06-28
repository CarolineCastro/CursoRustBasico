ESTRUTURAS

Em em outras linguagens temos classes, em rust temos structs.

    DECLARAÇÂO

    struct NomeStruct {
        variable1: type,
        variable2: type,
    }

    INSTANCIAMENTO

    let variable_struct = NomeStruct {
        variable1: value,
        variable2: value,
    };

    IMPRLEMENTAÇÃO

    impl NomeStruct {          
        fn new() -> Self { <- construtor
            Self {
                variable1: value,
                variable2: value,
            }
        }
    }

    let variable_struct = NomeStruct::new(); <- instanciando através do construtor


CARACTERÍSTICAS

Em outras linguagens temos as interfaces, em rust temos traits.

    DECLARAÇÃO

    trait TraitName {
        fn function(&self) -> &str
    }

    IMPLEMENTAÇÃO

    impl TraitName for StructName {
        fn function(&self) -> &str {
            //do stuff
        }
    }

Com traits podemos escrever funções genéricas que aceitam qualquer valor que implementa a trait.

    fn print_fn<T: TraitName>(item: T){
        println!("{}", item.function());
    }
    Essa função está definida como do tipo T que é qualquer tipo implementado pela trait TraitName.

