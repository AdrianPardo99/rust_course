#[allow(dead_code)]
#[allow(unused_variables)]
use std::mem; /* Llamada a funciones en std */
fn main(){
    let a:char='a';
    println!("Caracter: {} con {} bytes de uso",a,mem::size_of_val(&a));
    let b:f32=2.5;
    println!("Numero {} con {} bytes de uso",b,mem::size_of_val(&b));
    /* Forma de implementar un f64 */
    let c=2.5;
    println!("Numero {} con {} bytes de uso",c,mem::size_of_val(&c));
    let d:bool=true;
    println!("Booleano {} con {} bytes de uso en programa",d,mem::size_of_val(&d));
}
