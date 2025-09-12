Poryecto con Leptos sin SSR solo con CSR.. en planes mejorarlo a SSR

Crear nuetro proyecto
cargo new vnc

Ejecutar sitios CSR
cargo install trunk

TRABAJAR CON LEPTOS SOLO CON CSR
cargo add leptos --features=csr

Formateador de codigo por el momento, en el futuro ya habra uno oficial mas establecido
cargo install leptosfmt

Formatear todo el codigo que hacemos en leptos ya que muchas veces de desordena
leptosfmt .

Formatear un archivo en especifico
leptosfmt src/main.rs

Crear un index.html en la raiz

Un “hook” para que, cuando tu programa en Rust haga panic!, el mensaje se muestre claramente en la consola del navegador.
cargo add console_error_panic_hook

Configuracion es nuestro settings.json

{
"editor.formatOnSave": true,
..............
"rust-analyzer.cargo.allFeatures": true
"rust-analyzer.procMacro.ignored": {
"leptos_macro": [
"server"
],
},
"rust-analyzer.cargo.allFeatures": true,
"rust-analyzer.procMacro.enable": true
}

trunk serve --port 3000 --open

Preparamos tailwind

version: "4.1.13"
