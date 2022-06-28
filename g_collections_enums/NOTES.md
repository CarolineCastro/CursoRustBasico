COLEÇÕES

    VECTORS

    Comportamento de listas.

        DECLARAÇÃO

        let mut v: Vec<i32> = Vec::new();

        v.push(2);
        
        let x = v.pop();

        MACRO VECTORS

        Vetores com valores literais e mais ergonômicos.

            let mut v = vec![2, 4, 6];

    Vetores te dão vários controles de baixo nível, em segurança. Tem vários métodos previamente implementados para manipulá-los.

    HASMAPS

    É um coleção genérica em que você irá especificar um tipo para a chave e um tipo para o valor.
    Em algumas linguagens é chamado de diretório.
    O ponto principal do hashmap é estar apto para inserir, observar e remover valores por uma chave em tempo constante.

    HashMap<K, V>

        DECLARAÇÃO

            let mut h: HashMap<u8, bool> = HashMap::new();

            h.insert(5, true);

            let have_five = h.remove(&5).unwrap();
    
    VECDEQUE -> VecDeque lista q permite inserção e remoção de item no final e no início da lista. Porém todo o resto é um pouco menos eficiênte do que o Vector tradicional.

    LINKEDLIST -> LinkedList é uma lista que permite inserção e remoção de qualquer ponto, seja início, meio ou fim. Porém deixa a desejar em todo o resto.

    HASHSET -> HashSet é a implementação de uma definição que perfomam definições de operações realmente eficientes.

    BINARYHEAP -> BinaryHeap é como uma fila prioritária que sempre sai do valor máximo.

    BTREEMAP e BTREESET -> BTreeMap e BTreetSet são mapas alternados e difinições de implementações usando uma modificação de árvore binária. Usualmnete você só irá escolher entre eles se precisar do mapa de chaves ou definição de valores para sempre ser classificado.

ENUMS

Enums em Rust é como um tipo de dado algébrico de Haskell do que os enums do C.

    DECLARAÇÃO

        enum NameEnum {
            Variable1,
            Variable2,
            Variable3,
        }

    INSTÂNCIA

        let variable_enum = NameEnum::Variable1

    EXEMPLO

        enum DispenserItem{
            Empty,
            Ammo(u8),
            Thigs(String, i32),
            Place{x: i32, y: i32},
        }
        
        use DispenserItem::*;
        let item = Empty;

    IMPLEMENTAÇÃO

        impl DispenserItem {
            fn display(&self){
                //do stuff
            }
        }

Enums também podem ser genéricos.

    EXEMPLO

        enum Option<T> {
            Some(T),
            None,
        }

        Como enums representam todos os tipos de dados, é precio usar padrões para examiná-los. Se quiser checar por uma única variante, se usa o if let:

            if let Some(x) = my_variable{
                println!("Value is {}", x);
            }

        Mas quando é preciso checar todas as variantes de uma vez, usamos o match:

            match my_variable{
                Some(x) => {
                    println!("value is {}", x);
                },
                None => {
                    println!("no value");
                },
            }  

        Também podemos usar matches genéricos:

            match my_variable{
                _=> {
                    println!("who cares");
                },
            }
        
        Matches também podem ser atribuídos a variáveis.

            let x = match my_variable{
                _ =>{
                    println!("who cares");
                },
            }

        Outra forma de instanciar enums é pela sintaxe:

            let mut x: Option<i32> = None;
 


        