#[allow(dead_code)]
#[allow(unused_variables)]
use std::mem;
fn main(){
    let a:u8=123; /* Valor inmutable de sin signo de 8 bits */
    println!("Valor de a={} con {} bits sin signo",a,mem::size_of_val(&a)*8);
    let mut b:i8=-7; /* Valor mutable con signo de 8 bits */
    println!("Valor de b={} antes con {} bits con signo",b,mem::size_of_val(&b)*8);
    b=7;
    println!("Valor de b={} despues con {} bits con signo",b,mem::size_of_val(&b)*8);
    /* Tercera forma de declarar numeros */
    let c=5800;
    println!("Valor de c={} con {} bits",c,mem::size_of_val(&c)*8);
}
