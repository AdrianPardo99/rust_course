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
