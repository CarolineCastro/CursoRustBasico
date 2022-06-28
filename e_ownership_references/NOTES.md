PROPRIEDADE

É o que torna o Rust uma linguagem segura e rápida. 
Há 3 regras de propriedade:
1. Todo valor tem um dono. 
2. Só há apenas um único dono para cada valor.
3. Os valores serão descartados se o dono estiver fora do escopo.

    EXEMPLO 1:

    let s1 = String::from("abc);
    let s2 = s1; <- o valor de s1 não foi copiado para s2 e sem movido para lá. A partir desse momento o s1 se torna inútil, pois não é uma variável mutável.

    Isso acontece pois o ponteiro que apontava para o conteúdo de s1, foi substituído pelo ponteiro de s2.
    Para copiar o conteúdo sem substituir o ponteiro é preciso declarar como:

        let s2 = s1.clone();

    Assim o s2 se torna um clone do s1, sem substituí-lo.

    EXEMPLO 2:

    let s1 = String::from("abc");
    do_stuff(s1);

    fn do_stuff(s: String){
        //do stuff
    }

    Nesse contexto acontece o mesmo que no primeiro exemplo, pois s1 foi passado para s e não será mais possível usá-lo
    Para consertar isso é priciso tornar s1 mutável, assim após o uso do conteúdo pela função, a variável ainda poderá ser utilizada.

    let mut s1 = String::from("abc");
    s1 = do_stuff(s1);

    fn do_stuff(s: String) -> String{
        s
    }

    Mas isso não é uma boa prática em Rust.


REFERÊNCIA E EMPRÉSTIMO

Continuando do último exemplo de propriedade. 

let s1 = String::from("abc");
    do_stuff(&s1); <- dessa vez passamos a referência para a função e não o valor

    fn do_stuff(s: &String){
        //do stuff ^ assim  a função poderá usar o valor de s1 sem precisar movê-lo ou cloná-lo.
}

Mas se precisar retornar o valor alterado de alguma forma é preciso passar uma referência mutável, que se faz com a sintaxe &mut

let s1 = String::from("abc");
    do_stuff(&mut s1);

    fn do_stuff(s: &mut String){
        *s.insert_str(0, "hi, ");
    }
É preciso usar o * para indicar q s irá receber uma referência. Que é imutável.