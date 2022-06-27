#![allow(dead_code, unused_variables)]

use simple_types::{on_off, print_array, print_difference, print_distance, ding}; //using a libary for function out of main

fn main() {

    let coords: (f32, f32) = (6.3, 15.0);  //tuple
    print_difference(coords.0, coords.1);  //using tuple indexing
    
    let coords_arr: [f32; 2] = [coords.0, coords.1]; //transforming the tuple in array
    print_array(coords_arr);

    let series = [1, 1, 2, 3, 5, 8, 13]; //array
    ding(series[6]); //using array indexing

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy"); //tuple of mutiple types
    on_off(mess.2[1].0); //indexing a tuple
    //          ^ position 2 of tuple is a array, so index the array. But the array is a tuple too, so index the tuple outside the indexig of array.

    print_distance(coords.0, coords.1); //make this be accordingly with best pratices
}

/* All functions are moved to libary file
fn print_difference(x: f32, y: f32){
    println!("Difference between {} and {} is {}", x, y, (x - y).abs());
}

fn print_array(a: [f32; 2]){
    println!("The coordinates are ({}, {})", a[0], a[1]);
}

fn ding(x: i32){
    if x == 13 {
        println!("Ding, you found 13!");
    }
}

fn on_off(val: bool){
    if val {
        println!("Lights are on!");
    }
}

fn print_distance(x: f32, y: f32){

    println!(
        "Distance to the origin is {}",
        (x.powf(2.0) + y.powf(2.0)).sqrt());
}*/

