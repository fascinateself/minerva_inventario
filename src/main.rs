use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

const DB_PATH: &str = "inventario.db";

// ==========================================
// MÓDULO 1: ARREGLO DINÁMICO (Dynamic Array)
// ==========================================
#[derive(Clone)]
pub struct DynamicArray<T> {
    buffer: Box<[Option<T>]>,
    length: usize,
    capacity: usize,
}

impl<T: Clone> DynamicArray<T> {
    pub fn new(initial_capacity: usize) -> Self {
        let cap = if initial_capacity == 0 { 2 } else { initial_capacity };
        let buffer = vec![None; cap].into_boxed_slice();
        Self { buffer, length: 0, capacity: cap }
    }

    fn grow(&mut self) {
        if self.length == self.capacity {
            let new_capacity = (self.capacity * 1618) / 1000 + 1;
            let mut new_buffer = vec![None; new_capacity].into_boxed_slice();
            for i in 0..self.length {
                new_buffer[i] = self.buffer[i].take();
            }
            self.buffer = new_buffer;
            self.capacity = new_capacity;
        }
    }

    pub fn append(&mut self, data: T) {
        self.grow();
        self.buffer[self.length] = Some(data);
        self.length += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.length { self.buffer[index].as_ref() } else { None }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.length { self.buffer[index].as_mut() } else { None }
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.length { return None; }
        let removed = self.buffer[index].take();
        for i in index..self.length - 1 {
            self.buffer[i] = self.buffer[i + 1].take();
        }
        self.length -= 1;
        removed
    }

    pub fn insert(&mut self, index: usize, data: T) {
        if index >= self.length {
            self.append(data);
            return;
        }
        self.grow();
        for i in (index..self.length).rev() {
            self.buffer[i + 1] = self.buffer[i].take();
        }
        self.buffer[index] = Some(data);
        self.length += 1;
    }

    pub fn len(&self) -> usize { self.length }
    pub fn is_empty(&self) -> bool { self.length == 0 }
}

// ==========================================
// MÓDULO 2: LISTA ENLAZADA, PILA Y COLA
// ==========================================
#[derive(Clone)]
pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self { Node { data, next: None } }
}

#[derive(Clone)]
pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
    pub length: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self { LinkedList { head: None, length: 0 } }
    pub fn is_empty(&self) -> bool { self.head.is_none() }
    pub fn len(&self) -> usize { self.length }

    pub fn push_front(&mut self, data: T) {
        let mut new_node = Box::new(Node::new(data));
        new_node.next = self.head.take();
        self.head = Some(new_node);
        self.length += 1;
    }

    pub fn push_back(&mut self, data: T) {
        let new_node = Box::new(Node::new(data));
        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }
        *current = Some(new_node);
        self.length += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.length -= 1;
            node.data
        })
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.length { return None; }
        if index == 0 { return self.pop_front(); }
        
        let mut current = self.head.as_mut().unwrap();
        for _ in 0..index - 1 {
            current = current.next.as_mut().unwrap();
        }
        let removed_node = current.next.take().unwrap();
        current.next = removed_node.next;
        self.length -= 1;
        Some(removed_node.data)
    }

    pub fn insert(&mut self, index: usize, data: T) {
        if index == 0 {
            self.push_front(data);
            return;
        }
        if index >= self.length {
            self.push_back(data);
            return;
        }
        let mut current = self.head.as_mut().unwrap();
        for _ in 0..index - 1 {
            current = current.next.as_mut().unwrap();
        }
        let mut new_node = Box::new(Node::new(data));
        new_node.next = current.next.take();
        current.next = Some(new_node);
        self.length += 1;
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let mut current = self.head.as_mut();
        let mut i = 0;
        while let Some(node) = current {
            if i == index { return Some(&mut node.data); }
            current = node.next.as_mut();
            i += 1;
        }
        None
    }
}

// Implementación de impresión solo para tipos que soportan Display
impl<T: Display> LinkedList<T> {
    pub fn print(&self) {
        println!("(Total: {})", self.length);
        let mut current = &self.head;
        let mut i = 0;
        while let Some(node) = current {
            println!("  [{}] {}", i, node.data);
            current = &node.next;
            i += 1;
        }
    }
}

pub struct Queue<T> { pub list: LinkedList<T> }
impl<T> Queue<T> {
    pub fn new() -> Self { Queue { list: LinkedList::new() } }
    pub fn add(&mut self, data: T) { self.list.push_back(data); }
    pub fn pop(&mut self) -> Option<T> { self.list.pop_front() }
    pub fn is_empty(&self) -> bool { self.list.is_empty() }
}

pub struct Stack<T> { pub list: LinkedList<T> }
impl<T> Stack<T> {
    pub fn new() -> Self { Stack { list: LinkedList::new() } }
    pub fn push(&mut self, data: T) { self.list.push_front(data); }
    pub fn pop(&mut self) -> Option<T> { self.list.pop_front() }
}

// ==========================================
// MÓDULO 3: MODELOS DE DOMINIO (Inventario)
// ==========================================
#[derive(Clone, PartialEq, PartialOrd)]
pub enum TemplateAttribute {
    Text { title: String },
    SingleTag { category: String, color: String },
    TagSlider { label: String, min_label: String, max_label: String },
    Checkbox { label: String },
}

impl Display for TemplateAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            TemplateAttribute::Text { title } => write!(f, "[Texto: {}]", title),
            TemplateAttribute::SingleTag { category, color } => write!(f, "[Tag {}: {}]", category, color),
            TemplateAttribute::TagSlider { label, min_label, max_label } => write!(f, "[Slider {}: {} a {}]", label, min_label, max_label),
            TemplateAttribute::Checkbox { label } => write!(f, "[Checkbox: {}]", label),
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd)]
pub enum ObjectValue {
    Text(String),
    SingleTag(String),
    TagSlider(u8),
    Checkbox(bool),
}

impl Display for ObjectValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ObjectValue::Text(val) => write!(f, "{}", val),
            ObjectValue::SingleTag(val) => write!(f, "{}", val),
            ObjectValue::TagSlider(val) => write!(f, "{}%", val),
            ObjectValue::Checkbox(val) => write!(f, "{}", if *val { "Sí" } else { "No" }),
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd)]
pub struct AttributeValue {
    pub value: ObjectValue,
    pub visible: bool,
}

impl Display for AttributeValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let status = if self.visible { "VISIBLE" } else { "OCULTO" };
        write!(f, "{} [{}]", self.value, status)
    }
}

#[derive(Clone)]
pub struct InventoryObject {
    pub id: String,
    pub title: String,
    pub fallback_images: DynamicArray<String>, 
    pub attributes: LinkedList<AttributeValue>,
}

#[derive(Clone)]
pub struct Folder {
    pub name: String,
    pub template: LinkedList<TemplateAttribute>,
    pub objects: DynamicArray<InventoryObject>,
}

// ==========================================
// MÓDULO 4: MOTOR DE PERSISTENCIA
// ==========================================
pub struct PersistenceEngine;

impl PersistenceEngine {
    pub fn save_to_file(folders: &DynamicArray<Folder>, filepath: &str) -> std::io::Result<()> {
        let mut file = File::create(filepath)?;
        for i in 0..folders.len() {
            if let Some(folder) = folders.get(i) {
                writeln!(file, "FOLDER|{}", folder.name)?;
                let mut current_attr = &folder.template.head;
                while let Some(node) = current_attr {
                    match &node.data {
                        TemplateAttribute::Text { title } => writeln!(file, "ATTR_TEXT|{}", title)?,
                        TemplateAttribute::TagSlider { label, min_label, max_label } => writeln!(file, "ATTR_SLIDER|{}|{}|{}", label, min_label, max_label)?,
                        TemplateAttribute::Checkbox { label } => writeln!(file, "ATTR_CHECK|{}", label)?,
                        TemplateAttribute::SingleTag { category, color } => writeln!(file, "ATTR_TAG|{}|{}", category, color)?,
                    }
                    current_attr = &node.next;
                }
                for j in 0..folder.objects.len() {
                    if let Some(obj) = folder.objects.get(j) {
                        writeln!(file, "OBJECT|{}|{}", obj.id, obj.title)?;
                        let mut current_val = &obj.attributes.head;
                        while let Some(v_node) = current_val {
                            let vis = v_node.data.visible;
                            match &v_node.data.value {
                                ObjectValue::Text(t) => writeln!(file, "VAL_TEXT|{}|{}", t, vis)?,
                                ObjectValue::TagSlider(s) => writeln!(file, "VAL_SLIDER|{}|{}", s, vis)?,
                                ObjectValue::Checkbox(c) => writeln!(file, "VAL_CHECK|{}|{}", c, vis)?,
                                ObjectValue::SingleTag(tag) => writeln!(file, "VAL_TAG|{}|{}", tag, vis)?,
                            }
                            current_val = &v_node.next;
                        }
                        writeln!(file, "END_OBJECT")?;
                    }
                }
                writeln!(file, "END_FOLDER")?;
            }
        }
        Ok(())
    }

    pub fn load_from_file(filepath: &str) -> std::io::Result<DynamicArray<Folder>> {
        let file = match File::open(filepath) {
            Ok(f) => f,
            Err(_) => return Ok(DynamicArray::new(5)),
        };
        let reader = BufReader::new(file);
        let mut folders = DynamicArray::new(5);
        let mut current_folder: Option<Folder> = None;
        let mut current_object: Option<InventoryObject> = None;

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split('|').collect();
            if parts.is_empty() { continue; }

            match parts[0] {
                "FOLDER" => current_folder = Some(Folder { name: parts[1].to_string(), template: LinkedList::new(), objects: DynamicArray::new(5) }),
                "ATTR_TEXT" => if let Some(f) = current_folder.as_mut() { f.template.push_back(TemplateAttribute::Text { title: parts[1].to_string() }); },
                "ATTR_SLIDER" => if let Some(f) = current_folder.as_mut() { f.template.push_back(TemplateAttribute::TagSlider { label: parts[1].to_string(), min_label: parts[2].to_string(), max_label: parts[3].to_string() }); },
                "ATTR_CHECK" => if let Some(f) = current_folder.as_mut() { f.template.push_back(TemplateAttribute::Checkbox { label: parts[1].to_string() }); },
                "ATTR_TAG" => if let Some(f) = current_folder.as_mut() { f.template.push_back(TemplateAttribute::SingleTag { category: parts[1].to_string(), color: parts[2].to_string() }); },
                "OBJECT" => current_object = Some(InventoryObject { id: parts[1].to_string(), title: parts[2].to_string(), fallback_images: DynamicArray::new(2), attributes: LinkedList::new() }),
                "VAL_TEXT" => if let Some(o) = current_object.as_mut() { o.attributes.push_back(AttributeValue { value: ObjectValue::Text(parts[1].to_string()), visible: parts.get(2).and_then(|v| v.parse().ok()).unwrap_or(true) }); },
                "VAL_SLIDER" => if let Some(o) = current_object.as_mut() { o.attributes.push_back(AttributeValue { value: ObjectValue::TagSlider(parts[1].parse().unwrap_or(0)), visible: parts.get(2).and_then(|v| v.parse().ok()).unwrap_or(true) }); },
                "VAL_CHECK" => if let Some(o) = current_object.as_mut() { o.attributes.push_back(AttributeValue { value: ObjectValue::Checkbox(parts[1].parse().unwrap_or(false)), visible: parts.get(2).and_then(|v| v.parse().ok()).unwrap_or(true) }); },
                "VAL_TAG" => if let Some(o) = current_object.as_mut() { o.attributes.push_back(AttributeValue { value: ObjectValue::SingleTag(parts[1].to_string()), visible: parts.get(2).and_then(|v| v.parse().ok()).unwrap_or(true) }); },
                "END_OBJECT" => if let Some(obj) = current_object.take() { if let Some(f) = current_folder.as_mut() { f.objects.append(obj); } },
                "END_FOLDER" => if let Some(f) = current_folder.take() { folders.append(f); },
                _ => {}
            }
        }
        Ok(folders)
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn default_object_value(attr: &TemplateAttribute) -> ObjectValue {
    match attr {
        TemplateAttribute::Text { .. } => ObjectValue::Text("Sin especificar".to_string()),
        TemplateAttribute::TagSlider { .. } => ObjectValue::TagSlider(0),
        TemplateAttribute::Checkbox { .. } => ObjectValue::Checkbox(false),
        TemplateAttribute::SingleTag { .. } => ObjectValue::SingleTag("General".to_string()),
    }
}

// Función auxiliar para aplicar el "Deshacer"
fn handle_undo(folders: &mut DynamicArray<Folder>, undo_stack: &mut Stack<DynamicArray<Folder>>) -> bool {
    if let Some(prev_state) = undo_stack.pop() {
        *folders = prev_state;
        println!("\n[!] Acción deshecha correctamente. Estado anterior restaurado.");
        true
    } else {
        println!("\n[!] No hay acciones para deshacer.");
        false
    }
}

// ==========================================
// MÓDULO 5: CLI INTERACTIVO (Main)
// ==========================================
fn main() {
    let mut folders = match PersistenceEngine::load_from_file(DB_PATH) {
        Ok(data) if !data.is_empty() => {
            println!("Se cargó el estado guardado correctamente desde {}", DB_PATH);
            data
        }
        _ => {
            println!("No se encontró estado previo. Inicializando carpeta por defecto...");
            let mut init_folders = DynamicArray::new(5);
            let mut init_template = LinkedList::new();
            init_template.push_back(TemplateAttribute::Text { title: "Descripción".to_string() });
            init_folders.append(Folder { name: "General".to_string(), template: init_template, objects: DynamicArray::new(5) });
            init_folders
        }
    };

    // Pila Global de Deshacer
    let mut undo_stack: Stack<DynamicArray<Folder>> = Stack::new();

    loop {
        println!("\n=======================================================");
        println!("   GESTOR DE CARPETAS DE INVENTARIO                    ");
        println!("=======================================================");
        println!("Carpetas disponibles:");
        for i in 0..folders.len() {
            if let Some(f) = folders.get(i) {
                println!("  [{}] {} ({} objetos)", i, f.name, f.objects.len());
            }
        }
        
        println!("\n1. Entrar a una Carpeta");
        println!("2. Crear Nueva Carpeta");
        println!("3. Renombrar Carpeta");
        println!("4. Reorganizar Carpetas");
        println!("5. Eliminar Carpeta (Borra TODO en su interior)");
        println!("6. Salir y Guardar");
        println!("Z. Deshacer última acción");
        
        match read_input("Selección: ").as_str() {
            "1" => {
                if let Ok(idx) = read_input("Índice de carpeta: ").parse::<usize>() {
                    if idx < folders.len() {
                        gestionar_carpeta(&mut folders, idx, &mut undo_stack);
                    } else { println!("Carpeta no encontrada."); }
                }
            }
            "2" => {
                let name = read_input("Nombre de la nueva carpeta: ");
                undo_stack.push(folders.clone());
                folders.append(Folder { name, template: LinkedList::new(), objects: DynamicArray::new(5) });
                println!("Carpeta creada.");
            }
            "3" => {
                if let Ok(idx) = read_input("Índice de carpeta: ").parse::<usize>() {
                    let new_name = read_input("Nuevo nombre: ");
                    undo_stack.push(folders.clone());
                    if let Some(folder) = folders.get_mut(idx) {
                        folder.name = new_name;
                    }
                }
            }
            "4" => {
                if let Ok(from) = read_input("Mover carpeta (índice origen): ").parse::<usize>() {
                    if let Ok(to) = read_input("Hacia (índice destino): ").parse::<usize>() {
                        undo_stack.push(folders.clone());
                        if let Some(folder) = folders.remove(from) {
                            folders.insert(to, folder);
                            println!("Carpeta movida exitosamente.");
                        } else { println!("Índice de origen no válido."); }
                    }
                }
            }
            "5" => {
                if let Ok(idx) = read_input("Índice de carpeta a eliminar: ").parse::<usize>() {
                    let confirm = read_input("¿Está seguro? Se perderá la plantilla y TODOS sus objetos. (S/N): ").to_uppercase();
                    if confirm == "S" {
                        undo_stack.push(folders.clone());
                        if let Some(removed) = folders.remove(idx) {
                            println!("Carpeta '{}' destruida con todos sus elementos.", removed.name);
                        }
                    }
                }
            }
            "6" => break,
            "Z" | "z" => { handle_undo(&mut folders, &mut undo_stack); },
            _ => println!("Opción no válida."),
        }
    }

    if let Err(e) = PersistenceEngine::save_to_file(&folders, DB_PATH) {
        println!("Error al guardar el estado: {}", e);
    } else {
        println!("Estado guardado exitosamente en {}", DB_PATH);
    }
}

// Se pasa el arreglo completo de carpetas y el índice para poder clonar el estado global antes de cualquier mutación
fn gestionar_carpeta(folders: &mut DynamicArray<Folder>, folder_idx: usize, undo_stack: &mut Stack<DynamicArray<Folder>>) {
    loop {
        let folder_name = folders.get(folder_idx).unwrap().name.clone();
        println!("\n--- CARPETA: {} ---", folder_name);
        println!("1. GESTIONAR PLANTILLA");
        println!("2. GESTIONAR OBJETOS");
        println!("3. Volver al menú principal");
        println!("Z. Deshacer última acción");
        
        match read_input("Selección: ").as_str() {
            "1" => {
                loop {
                    let folder_name = folders.get(folder_idx).unwrap().name.clone();
                    println!("\n-- PLANTILLA DE {} --", folder_name);
                    println!("1. Ver Plantilla");
                    println!("2. Agregar Atributo (Se agrega OCULTO a objetos existentes)");
                    println!("3. Eliminar Atributo (Cascada)");
                    println!("4. Reorganizar Atributo");
                    println!("5. Volver");
                    println!("Z. Deshacer última acción");
                    
                    match read_input("Opción: ").as_str() {
                        "1" => folders.get(folder_idx).unwrap().template.print(),
                        "2" => {
                            println!("\na. Texto | b. Tag Slider | c. Checkbox | d. Single Tag");
                            let tipo = read_input("Tipo: ");
                            if !["a", "b", "c", "d"].contains(&tipo.as_str()) { continue; }
                            
                            let attr = match tipo.as_str() {
                                "a" => TemplateAttribute::Text { title: read_input("Título: ") },
                                "b" => TemplateAttribute::TagSlider { label: read_input("Label: "), min_label: read_input("Min: "), max_label: read_input("Max: ") },
                                "c" => TemplateAttribute::Checkbox { label: read_input("Label: ") },
                                "d" => TemplateAttribute::SingleTag { category: read_input("Categoría: "), color: read_input("Color Hex: ") },
                                _ => unreachable!(),
                            };
                            
                            undo_stack.push(folders.clone());
                            let folder = folders.get_mut(folder_idx).unwrap();
                            folder.template.push_back(attr.clone());

                            let default_val = default_object_value(&attr);
                            for i in 0..folder.objects.len() {
                                if let Some(obj) = folder.objects.get_mut(i) {
                                    obj.attributes.push_back(AttributeValue { value: default_val.clone(), visible: false });
                                }
                            }
                            println!("¡Atributo agregado a la plantilla y acoplado (oculto) en todos los objetos!");
                        }
                        "3" => {
                            folders.get(folder_idx).unwrap().template.print();
                            if let Ok(idx) = read_input("Índice a eliminar: ").parse::<usize>() {
                                if idx < folders.get(folder_idx).unwrap().template.len() {
                                    let confirm = read_input("¿Confirmar eliminación? Se borrará el valor en TODOS los objetos (S/N): ").to_uppercase();
                                    if confirm == "S" {
                                        undo_stack.push(folders.clone());
                                        let folder = folders.get_mut(folder_idx).unwrap();
                                        if let Some(removed) = folder.template.remove(idx) {
                                            for i in 0..folder.objects.len() {
                                                if let Some(obj) = folder.objects.get_mut(i) {
                                                    obj.attributes.remove(idx);
                                                }
                                            }
                                            println!("Atributo eliminado: {}", removed);
                                        }
                                    }
                                }
                            }
                        }
                        "4" => {
                            if let Ok(from) = read_input("Mover (índice): ").parse::<usize>() {
                                if let Ok(to) = read_input("Hacia (índice): ").parse::<usize>() {
                                    undo_stack.push(folders.clone());
                                    let folder = folders.get_mut(folder_idx).unwrap();
                                    if let Some(removed) = folder.template.remove(from) {
                                        folder.template.insert(to, removed);
                                        for i in 0..folder.objects.len() {
                                            if let Some(obj) = folder.objects.get_mut(i) {
                                                if let Some(obj_val) = obj.attributes.remove(from) {
                                                    obj.attributes.insert(to, obj_val);
                                                }
                                            }
                                        }
                                        println!("Plantilla y objetos reorganizados.");
                                    }
                                }
                            }
                        }
                        "5" => break,
                        "Z" | "z" => { handle_undo(folders, undo_stack); },
                        _ => println!("Inválido."),
                    }
                }
            }
            "2" => {
                loop {
                    let folder_name = folders.get(folder_idx).unwrap().name.clone();
                    println!("\n-- OBJETOS DE {} --", folder_name);
                    println!("1. Ver Lista (Atributos Visibles / Todos)");
                    println!("2. Crear Objeto");
                    println!("3. Editar Objeto (Valor / Toggle Visibilidad)");
                    println!("4. Eliminar Objeto");
                    println!("5. Volver");
                    println!("Z. Deshacer última acción");
                    
                    match read_input("Opción: ").as_str() {
                        "1" => {
                            let show_all = read_input("¿Mostrar también atributos ocultos? (S/N): ").to_uppercase() == "S";
                            let folder = folders.get(folder_idx).unwrap();
                            for i in 0..folder.objects.len() {
                                if let Some(obj) = folder.objects.get(i) {
                                    println!("\n[{}] ID: {} | Título: {}", i, obj.id, obj.title);
                                    let mut current = &obj.attributes.head;
                                    let mut idx = 0;
                                    while let Some(node) = current {
                                        if show_all || node.data.visible {
                                            println!("    [{}] {}", idx, node.data);
                                        }
                                        current = &node.next;
                                        idx += 1;
                                    }
                                }
                            }
                        }
                        "2" => {
                            let id = read_input("ID del Objeto: ");
                            let title = read_input("Título: ");
                            let mut obj_values = LinkedList::new();
                            
                            let folder = folders.get(folder_idx).unwrap();
                            let mut current = &folder.template.head;
                            while let Some(node) = current {
                                match &node.data {
                                    TemplateAttribute::Text { title } => obj_values.push_back(AttributeValue { value: ObjectValue::Text(read_input(&format!("{}: ", title))), visible: true }),
                                    TemplateAttribute::TagSlider { label, min_label, max_label } => {
                                        let val = loop { if let Ok(v) = read_input(&format!("{} ({} a {} -> 0-100): ", label, min_label, max_label)).parse::<u8>() { if v <= 100 { break v; } } };
                                        obj_values.push_back(AttributeValue { value: ObjectValue::TagSlider(val), visible: true });
                                    },
                                    TemplateAttribute::Checkbox { label } => {
                                        let b = loop { let v = read_input(&format!("{} (S/N): ", label)).to_uppercase(); if v == "S" { break true; } if v == "N" { break false; } };
                                        obj_values.push_back(AttributeValue { value: ObjectValue::Checkbox(b), visible: true });
                                    },
                                    TemplateAttribute::SingleTag { category, .. } => obj_values.push_back(AttributeValue { value: ObjectValue::SingleTag(read_input(&format!("Tag para {}: ", category))), visible: true }),
                                }
                                current = &node.next;
                            }
                            
                            undo_stack.push(folders.clone());
                            folders.get_mut(folder_idx).unwrap().objects.append(InventoryObject { id, title, fallback_images: DynamicArray::new(2), attributes: obj_values });
                            println!("Objeto creado en la carpeta.");
                        }
                        "3" => {
                            if let Ok(idx) = read_input("Índice de objeto a editar: ").parse::<usize>() {
                                let folder = folders.get_mut(folder_idx).unwrap();
                                if idx < folder.objects.len() {
                                    // Guardamos estado antes de iniciar la edición profunda
                                    undo_stack.push(folders.clone());
                                    
                                    // Volvemos a obtener el folder mutable tras el clone global
                                    let folder_mut = folders.get_mut(folder_idx).unwrap();
                                    if let Some(obj) = folder_mut.objects.get_mut(idx) {
                                        println!("Atributos del objeto:");
                                        obj.attributes.print();
                                        
                                        if let Ok(attr_idx) = read_input("Índice de atributo: ").parse::<usize>() {
                                            if let Some(attr_ref) = obj.attributes.get_mut(attr_idx) {
                                                println!("1. Cambiar Valor");
                                                println!("2. Alternar Visibilidad (Actual: {})", if attr_ref.visible { "VISIBLE" } else { "OCULTO" });
                                                
                                                match read_input("Acción: ").as_str() {
                                                    "1" => {
                                                        match &mut attr_ref.value {
                                                            ObjectValue::Text(s) => *s = read_input("Nuevo Texto: "),
                                                            ObjectValue::TagSlider(num) => *num = read_input("Nuevo valor (0-100): ").parse().unwrap_or(*num),
                                                            ObjectValue::Checkbox(b) => *b = !*b,
                                                            ObjectValue::SingleTag(t) => *t = read_input("Nuevo Tag: "),
                                                        }
                                                        println!("Valor actualizado.");
                                                    }
                                                    "2" => {
                                                        attr_ref.visible = !attr_ref.visible;
                                                        println!("Visibilidad cambiada a: {}", if attr_ref.visible { "VISIBLE" } else { "OCULTO" });
                                                    }
                                                    _ => { undo_stack.pop(); println!("Acción no válida."); }, // Revertir guardado si cancela
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        "4" => {
                            if let Ok(idx) = read_input("Índice de objeto a eliminar: ").parse::<usize>() {
                                undo_stack.push(folders.clone());
                                if let Some(removed) = folders.get_mut(folder_idx).unwrap().objects.remove(idx) {
                                    println!("Objeto {} eliminado.", removed.title);
                                }
                            }
                        }
                        "5" => break,
                        "Z" | "z" => { handle_undo(folders, undo_stack); },
                        _ => println!("Inválido."),
                    }
                }
            }
            "3" => break,
            "Z" | "z" => { handle_undo(folders, undo_stack); },
            _ => println!("Opción no válida."),
        }
    }
}