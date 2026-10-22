# Manual de Usuario: Gestor de Inventario Minerva

Este programa permite organizar, estructurar y persistir inventarios personalizados mediante carpetas, plantillas flexibles y objetos con atributos dinámicos.

## Requisitos y Primeros Pasos

### Requisitos previos
* Tener instalado **Rust** (con `cargo`) o utilizar el entorno de desarrollo preconfigurado mediante el archivo `.devcontainer/devcontainer.json` provisto en el repositorio.

### Cómo iniciar el programa
1. Clone el repositorio en su máquina local.
2. Abra una terminal en la raíz del proyecto.
3. Compile y ejecute el programa ejecutando el siguiente comando de Cargo:
   ```bash
   cargo run
   ```
4. Al iniciar, el programa buscará el archivo `inventario.db`. Si no existe, inicializará automáticamente una carpeta por defecto llamada **"General"** con un atributo de texto llamado **"Descripción"**.

---

## Menú Principal y Comandos Globales

Al iniciar la aplicación, se presenta el menú principal con la lista de carpetas disponibles y los siguientes comandos globales:

| Opción / Comando | Descripción |
| :--- | :--- |
| **`1` (Entrar a una Carpeta)** | Permite seleccionar una carpeta mediante su índice numérico para gestionar su plantilla y sus objetos internos. |
| **`2` (Crear Nueva Carpeta)** | Crea una nueva carpeta vacía solicitando un nombre al usuario. |
| **`3` (Renombrar Carpeta)** | Modifica el nombre de una carpeta existente seleccionada por su índice. |
| **`4` (Reorganizar Carpetas)** | Mueve una carpeta de una posición de origen (`from`) a una de destino (`to`). |
| **`5` (Eliminar Carpeta)** | **Acción destructiva:** Elimina por completo una carpeta, junto con su plantilla y **todos** los objetos que contiene. Requiere confirmación (`S/N`). |
| **`6` (Salir y Guardar)** | Guarda todo el estado actual del inventario en `inventario.db` y cierra la aplicación de forma segura. |
| **`Z` / `z` (Deshacer)** | Revierte la última acción realizada (creaciones, modificaciones, eliminaciones o reordenamientos). |

---

## Gestión de Carpetas: Plantillas y Objetos

Al ingresar a una carpeta (opción `1` del menú principal), se accede a un submenú que divide la gestión en dos áreas fundamentales: **Plantilla** y **Objetos**.

### Gestionar Plantilla
La plantilla define la estructura que tendrán todos los objetos dentro de esa carpeta.

| Opción / Comando | Descripción |
| ----- | ----- |
| **`1` (Entrar a una Carpeta)** | Permite seleccionar una carpeta mediante su índice numérico para gestionar su plantilla y sus objetos internos. |
| **`2` (Crear Nueva Carpeta)** | Crea una nueva carpeta vacía solicitando un nombre al usuario. |
| **`3` (Renombrar Carpeta)** | Modifica el nombre de una carpeta existente seleccionada por su índice. |
| **`4` (Reorganizar Carpetas)** | Mueve una carpeta de una posición de origen (`from`) a una de destino (`to`). |
| **`5` (Eliminar Carpeta)** | **Acción destructiva:** Elimina por completo una carpeta, junto con su plantilla y **todos** los objetos que contiene. Requiere confirmación (`S/N`). |
| **`6` (Salir y Guardar)** | Guarda todo el estado actual del inventario en `inventario.db` y cierra la aplicación de forma segura. |
| **`Z` / `z` (Deshacer)** | Revierte la última acción realizada (creaciones, modificaciones, eliminaciones o reordenamientos). |

### Gestionar Objetos
Permite administrar los elementos almacenados bajo la estructura de la plantilla de la carpeta.

| Opción / Comando | Descripción |
| ----- | ----- |
| **`1` (Ver Lista)** | Muestra los objetos con su ID y Título. El sistema preguntará si desea *mostrar también los atributos ocultos* (`S/N`). |
| **`2` (Crear Objeto)** | Solicita un ID único, un Título y los valores correspondientes para cada uno de los atributos definidos en la plantilla de la carpeta. |
| **`3` (Editar Objeto)** | Permite modificar un objeto seleccionado por su índice. Permite **Cambiar Valor** (actualiza el dato según su tipo) o **Alternar Visibilidad** (cambia entre `VISIBLE` u `OCULTO`). |
| **`4` (Eliminar Objeto)** | Borra un objeto específico de la carpeta según su índice. |
| **`5` (Volver)** | Regresa al menú anterior de la carpeta. |
| **`Z` / `z` (Deshacer)** | Revierte la última modificación realizada sobre los objetos. |
