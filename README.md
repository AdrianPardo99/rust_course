# Curso de Rust
## Qué es?
Rust es un lenguaje de programación compilado, puede ser considerado como un lenguaje de propósito general o multiparadigma cuyo desarrollador es Mozilla.

Al ser multiparadigma el lenguaje soporta:
* Programación Funcional
* Programación Orientada a Objetos
* Programación por Procedimientos
* Programación Imperativa

<img src="https://github.com/AdrianPardo99/rust_course/imgs/dancing-ferris.gif" width=100>
## Instalación de Rust
Para segun la [página](https://www.rust-lang.org/tools/install) es solo necesario el uso de cURL para descargar e instalar solamente escribiendo en nuestra terminal:
```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Finalmente para el caso particular de Windows solo hay que [descargar el binario](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe) e instalar, posiblemente sea necesario crear las _env variables_.
<img src="https://github.com/AdrianPardo99/rust_course/imgs/rustacean-flat-gesture.png" width=100>
## Hola mundo
Para el clásico Hola Mundo es necesario saber que las funciones en el lenguaje son llamadas o nombradas como macros para la ejecución de código en particular.

Por ello la llamada _println!()_ es una macro que ejecuta nuestro lenguaje para de esta forma ser ejecutado.
```rust
  fn main(){
    println!("Hola mundo desde Rust");
  }
```
