CLOSURES

É uma função anônima que pode emprestar ou capturar algum dado do escopo  em que está aninhado.

    SINTAXE

        |x, y| {x + y}

        Não precisa de parâmentros se não quiser

            || {x + y}
        
        EXEMPLO

            let add = |x, y| {x + y};
            add(1,2);
        
Closers suportam a semântica move, é usado quando precisamos que o que foi feito pela closer seja usado em outra parte do código.

    let f = move || {
        println!("{}", s);
    };
    f();


THREADS

Em Rust segmentação é portátil. Isso signifca que seu código vai funcionar em qualquer sistema.

    EXEMPLO:

        use std::thread;

        fn main() {
            let handle = thread::spawn(move || { 
                //do stuff in a child thready
            });

            // do stuff simultaneously in the main thread

            //wait until thread has exited
            handle.join().unwrap();
        }

Segmentação é um pouco pesado. Criando uma nova segmentação aloca ao sistema operacional dependente uma quantidade RAM para a segmentação da própria pilha, ou seja, geralmente alguns megabytes. 
A qualquer momento a CPU muda a execução de um seguimento para outro, isso tem q ser feito de um contexto expansivo de mudança.
Então quanto mais segmentos você tiver tentando compartilhar o core de uma CPU, mais sobrecarga vai ter no contexto de mudança. Mesmo assim, seguimentos são uma ferramenta fantática quando se precisar usar a CPU e a memória simultaneamente, porque eles podem executar simultaneamente em múltiplos cores, e realmente realizar mais trabalho.



