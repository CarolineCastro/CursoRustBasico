CONTROLE DE FLUXO

        IF

if num == 5{                   
    msg = "five"
} else if num == 4{     
    msg = "four"
} else {
    msg = "other"
} 

        or 

msg = if num == 5{
    "five"
} else if num == 4{
    "four"
} else {
    "other"
};

        or

num = if a {b} else {c};  or  num = if a{
                                if x {y} else {z}
                            } else {c};


        LAÇOS DE REPETIÇÃO

    LOOP

É um loop sem condicional, precisam ser parados com break;

loop{
    break;
}
 or 
 
 'bob: loop{
    loop{
        break 'bob;
    }
 }

 or

 'bob: loop{
    loop{
        continue 'bob; -> vai interromper o loop interno e continuar o externo
    }
 }

    WHILE

while condicion {
    //do stuff
}

    FOR

for num in [7,8,9].inter(){  iter renorna a coleção em ordem se já estiver ordenada e aleatório se nn estiver
    //do stuff with num
}

or

for (x, y) in array.inter(){
    //do stuff with x and y
}

or 

for num in 0..50{ indo de 0 a 49, se colocar =50, vai até 50.

}


STRINGS

        &STR
O conteúdo não pode ser modificado

        String
O conteúdo pode ser modificado. É composto por &str
