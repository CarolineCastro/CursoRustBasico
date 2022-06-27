DECLARAÇÃO DE VARIÁVEIS:

    let variable = 2;
    ^
    indica a declaração de uma váriavel, seguida do nome e do valor. OBS: toda váriavel TEM que ser INICIALIZADA com um valor que indica seu TIPO.

    let variable: i32 = 2;
                   ^
                   declara o tipo da variável, no caso, inteiro de 32 bits.

A primeira forma de declarar já deixa a cargo do compilador enteder através do valor designado qual o tipo da variável, já a segunda forma explicíta qual tipo é.

OBS: VARIÁVEIS EM RUST SÃO IMUTÁVEIS POR PADRÃO

    let mut variable = 2;
        ^
        tornando a variável mutável, assim será possível mudar seu valor ao longo do escopo.

O mut deve ser usado apenas para variáveis que precisarão ter seus valores alterados ao longo da execução. 

    const CONSTANT: i32 = 2;
    ^
    indica uma constante, que deve ser declarada com todas as letras em maiúsculo e palavras separadas por _ underlines.

Esse tipo de declaração PRECISA do tipo explicitado.


    let (variable1, variable2) = (2, 3);
    ^
    declaração de variáveis na mesma linha

    let (variable1, variable2): (i32, i32) = (2, 3);
                                 ^
                                 declaração de variáveis na mesma linha explicitando o tipo de cada uma
    let (mut variable1, variable2) = (2, 3);
         ^
         declaração de variável mutável junto com uma imutável na mesma linha
    let mut (variable1, variable2) = (2, 3);
        ^ declaração de todas as variáveis mutáveis na mesma linha     

