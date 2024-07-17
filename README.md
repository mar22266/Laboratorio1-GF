# Laboratorio de Gráficos por Computadora

## Descripción del Proyecto
Este proyecto implementa un sistema para dibujar y rellenar polígonos utilizando Rust y la biblioteca nalgebra_glm. El objetivo es demostrar la capacidad de manejar gráficos básicos mediante la manipulación directa de un framebuffer.

## Polígonos Implementados
- **Polígono 1**: Dibujado con color amarillo y borde blanco.
- **Polígono 2**: Dibujado con color azul y borde blanco.
- **Polígono 3**: Dibujado con color rojo y borde blanco, introduciendo el concepto de relleno complejo.
- **Polígono 4**: Dibujado con color verde y borde blanco, incluyendo el Polígono 5 como un agujero dentro de él.
- **Polígono 5**: Actúa como un agujero dentro del Polígono 4.

## Tecnologías Utilizadas
- **Rust**: Lenguaje de programación enfocado en seguridad y rendimiento.
- **nalgebra_glm**: Biblioteca utilizada para facilitar operaciones matemáticas necesarias para la manipulación gráfica.

## Estructura del Código
El código se divide en varios módulos:
- `framebuffer.rs`: Maneja las operaciones directas sobre el framebuffer, incluyendo el dibujo de píxeles y polígonos.
- `line_impl.rs`: Implementa el algoritmo de Bresenham para el dibujo de líneas.
- `color.rs`, `bmp.rs`: Proporcionan funcionalidades adicionales para manejo de colores y exportación de imágenes.

## Instrucciones de Uso
Para ejecutar este proyecto, asegúrate de tener Rust y Cargo instalados. Luego, sigue estos pasos:
1. Clona el repositorio en tu máquina local.
2. Navega al directorio del proyecto.
3. Ejecuta el comando `cargo run` para compilar y ejecutar el proyecto.
4. Revisa la imagen de salida generada para ver los polígonos renderizados.

## Licencia
Este proyecto está licenciado bajo la Licencia MIT - ver el archivo LICENSE.md para más detalles.
