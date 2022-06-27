TIPOS SIMPLES:

    Inteiros:

Unsigned    Signed
u8          i8
u16         i16
u32         i32
u64         i64
u128        i128
usize       isize

OBS: usize e isize não tem tamanho defino pois serão usandos por ponteiros, ou seja, terão o tamanho do que o ponteiro está apontando

Legenda: tipo de inteiro número de bits

    Inteiros Literais:

Decimal 1000000
Hex     0xedeadbeef
Octal 0o77543211
Binary 0b11110011
Byte(u8 only) b'A'

Ponto Flutuante:

f32 e f64 OBS: cuidado ao usar o f64, pois nem toda máquina suporta 64 bits

Booleano:

true -> para usar valores é preciso declarar true as u8
false 

Caractere:

char -> 4bytes

declaração: let letter = 'a'



TIPOS COMPOSTOS:

        Tupla:
        
let info = (1, 3.3, 999);

    let info: (u8, f64, i32) = (1, 3.3, 999);

        let jets = info.0; or let (jets, full, other) = info;

Máximo de itens: 12

        Array:
        
let buf = [1, 2, 3];
    let buf [0; 3];
        let buf: [u8; 3] = [1, 2, 3];

Tamanho máximo: 32
