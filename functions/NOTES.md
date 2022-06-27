DECLARAÇÃO DE FUNÇÕES:

    fn name_function(variable1: i32, variable2: i32) -> i32
    ^             
    toda função deve ser declarada por fn, o nome dever ser em minúsculo e separar palavras com underline. As variávies precisam ter seus tipos explicitados e o tipo do retorno deve ser antecedido por ->

OBS: As boas práticas de programação em RUST desencorajam o uso do return por causa da redundância. 
        EXEMPLO:
            fn name_function(variable1: i32, variable2: i32) -> i32 {
                return 0;
                ^ isso é desnecessário, basta tirar o return e manter o valor a ser retornado sem colocar ; no final.
            }

            fn name_function(variable1: i32, variable2: i32) -> i32 {
                0
                ^ essa é a forma correta.
            }
