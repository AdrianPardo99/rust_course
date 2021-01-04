# Curso de Rust
## Qué es?
Rust es un lenguaje de programación compilado, puede ser considerado como un lenguaje de propósito general o multiparadigma cuyo desarrollador es Mozilla.

Al ser multiparadigma el lenguaje soporta:
* Programación Funcional
* Programación Orientada a Objetos
* Programación por Procedimientos
* Programación Imperativa

<img src="https://github.com/AdrianPardo99/rust_course/blob/master/imgs/dancing-ferris.gif" width=100>

## Instalación de Rust
Para segun la [página](https://www.rust-lang.org/tools/install) es solo necesario el uso de cURL para descargar e instalar solamente escribiendo en nuestra terminal:
```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Finalmente para el caso particular de Windows solo hay que [descargar el binario](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe) e instalar, posiblemente sea necesario crear las _env variables_.
<img src="https://github.com/AdrianPardo99/rust_course/blob/master/imgs/rustacean-flat-gesture.png" width=100>
## Hola mundo
Para el clásico Hola Mundo es necesario saber que las funciones en el lenguaje son llamadas o nombradas como macros para la ejecución de código en particular.

Por ello la llamada _println!()_ es una macro que ejecuta nuestro lenguaje para de esta forma ser ejecutado.
```rust
  fn main(){
    println!("Hola mundo desde Rust");
  }
```
Finalmente para compilarlo con el compilador de Rust solo es necesario realizar lo siguiente:
```bash
  rustc <archivo>
```
## Empaquetamiento y compilado con Cargo
Cargo es un Administrador de paquetes que te ayuda a compilar y crear los archivos binarios a partir de una archivo de descripción, por ello la organización de como se realiza esto es:
* Archivo Cargo.toml
* Organizar todos los archivos fuente en un directorio _src/_ y que el archivo principal se llame _main.rs_

La estructura del archivo es la siguiente:
```toml
  [package]
  name="nombre del binario"
  version="Version de la aplicación con tres numeros 0.0.0"
  author=["Información del desarrollador"]
```

Finalmente para la compilación de todos nuestros archivos solo pondremos:
```bash
  cargo build
```

Y finalmente para ejecutar
```bash
  cargo run
```
Estos comandos generaran un archivo _.lock_ y una carpeta la cual contendrá todo el compilado de nuestros archivos que se alojaran en _target/_
## Variables
Si bien en muchos lenguajes hay varios tipos de datos con los cuales podemos trabajar, en Rust se pueden crear variables desde la forma más sencilla, hasta la forma donde se específica totalmente el tipo de dato y cuantos bits utiliza en el programa.
### Tabla de valores predeterminados
__Valores con signo:__ En Rust es posible decirle a un valor, variable o constante el cuantos bits va utilizar en el programa con el fin de optimizar los recursos espaciales de nuestro programa, por ello la tabla para poner un valor con signo es _ix_ donde __x__ es el numero de bits que utilizara nuestro programa y la __i__ representa que el tipo de dato es con signo.

La representación de que valores cubre __x__ va bajo el siguiente intervalo <img src="https://render.githubusercontent.com/render/math?math=\left[-2^{\left(N-1\right)},2^{\left(N-1\right)}-1\right]">

De tal forma que la tabla de los valores que cubre __x__ es:
| Bits | Intervalo |
| ---- | --------- |
|   8     | -128 hasta 127 |
|   16    | -32768 hasta 32767 |
|   32    | -2147483648 hasta 2147483647 |
|   64    | -9223372036854775808 hasta 9223372036854775807 |
|  128    | -170141183460469231731687303715884105728 hasta 170141183460469231731687303715884105727 |
| isize   | Este va de acuerdo a la arquitectura en la que se desarrolla el programa |

__Valores sin signo:__ Si bien el uso de signo nos permite separar un lado positivo y negativo de los valores que puede tomar van del intervalo <img src="https://render.githubusercontent.com/render/math?math=\left[0,2^{\left(N-1\right)}\right]"> y para declarar este tipo de datos es necesario escribir _ux_ donde __u__ representa el tipo de dato sin signo.

De tal forma que la tabla y los intervalos que cubre es la siguiente:
| Bits | Intervalo |
| ---- | --------- |
| 8    | 0 hasta 255 |
| 16   | 0 hasta 65536 |
| 32   | 0 hasta 4294967296 |
| 64   | 0 hasta 18446744073709551616 |
| 128  | 0 hasta 340282366920938463463374607431768211456 |
| usize | Este va de acuerdo a la arquitectura en la que se desarrolla el programa |

Para utilizarlo en algún programa de Rust solo es necesario realizar lo siguiente:
```rust
  #[allow(dead_code)]
  #[allow(unused_variables)]
  use std::mem; /* Llamada a funciones en std */
  fn main(){
    let a:u8=123; /* Valor inmutable de sin signo de 8 bits */
    println!("Valor de a={} con {} bits sin signo",a,mem::size_of_val(&a)*8);
    let mut b:i8=-7; /* Valor mutable con signo de 8 bits */
    println!("Valor de b={} antes con {} bits con signo",b,mem::size_of_val(&b)*8);
    b=7;
    println!("Valor de b={} despues con {} bits con signo",b,mem::size_of_val(&b)*8);
    /* Tercera forma de declarar numeros */
    let c=5800;
    println!("Valor de c={} con {} bits",c,mem::size_of_val(&c));
  }
```
### Otros tipos de variables
También en Rust existen otro tipo de datos los cuales nos ayuda demasiado para trabajar con distintas cosas, como números punto flotante (Números reales), cadena de caracteres o caracteres en particular.

Por ello para declarar distintos tipos de datos tenemos lo siguiente:
| Prefijo | Tipo de dato |
| ---- | --------- |
| char    | Carácter     |
| f32     | Número de punto flotante de 32 bits |
| f64     | Número de punto flotante de 64 bits |
| bool    | Booleano solo toma valores _true_/_false_ |

Implementando en código:
```rust
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
```
