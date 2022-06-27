#![allow(unused_variables)]

//Exercise is fix the code

fn main() {

    let width = 4;
    let height = 7;
    let depth = 10;

    /* first error: the println cant see the variable area 
    because its is declare inside this scop
    {
        let area = area_of(width, height);
    }*/

    let area = area_of(width, height);

    println!("Area is {}", area); 

    println!("Volume is {}", volume(width, height, depth)); //third error: it didnt works beacause it didnt exist fn volume
}

fn area_of(x: i32, y:i32) -> i32 {
   // return 0; second error: area is not doing what they have to do because this
   //return x * y; this expression is not accordingly to best pratices

   /*challange*/ 
    x * y
}

fn volume(x: i32, y: i32, z:i32) -> i32{
    x * y * z
}
