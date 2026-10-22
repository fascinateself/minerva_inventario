use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::{self, Write};

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

impl<T: Display + PartialOrd> LinkedList<T> {
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

pub struct Stack<T> { pub list: LinkedList<T> }
impl<T: Display + PartialOrd> Stack<T> {
    pub fn new() -> Self { Stack { list: LinkedList::new() } }
    pub fn add(&mut self, data: T) { self.list.push_front(data); }
    pub fn pop(&mut self) -> Option<T> { self.list.pop_front() }
}

pub struct Queue<T> { pub list: LinkedList<T> }
impl<T: Display + PartialOrd> Queue<T> {
    pub fn new() -> Self { Queue { list: LinkedList::new() } }
    pub fn add(&mut self, data: T) { self.list.push_back(data); }
    pub fn pop(&mut self) -> Option<T> { self.list.pop_front() }
    pub fn is_empty(&self) -> bool { self.list.is_empty() }
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

#[derive(Clone)]
pub struct InventoryObject {
    pub id: String,
    pub title: String,
    pub fallback_images: DynamicArray<String>, 
    pub attributes: LinkedList<ObjectValue>,
}

#[derive(Clone)]
pub struct Folder {
    pub name: String,
    pub template: LinkedList<TemplateAttribute>,
    pub objects: DynamicArray<InventoryObject>,
}

pub enum ExportFormat { CSV }
pub struct ExportJob { pub object_id: String, pub format: ExportFormat, pub raw_data: String }

impl Display for ExportJob {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { write!(f, "Exportar {} a CSV", self.object_id) }
}
impl PartialOrd for ExportJob {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { self.object_id.partial_cmp(&other.object_id) }
}
impl PartialEq for ExportJob {
    fn eq(&self, other: &Self) -> bool { self.object_id == other.object_id }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// ==========================================
// MÓDULO 5: CLI INTERACTIVO (Main)
// ==========================================
fn main() {
    let mut export_queue: Queue<ExportJob> = Queue::new();
    let mut folders: DynamicArray<Folder> = DynamicArray::new(5);

    // Precargar una carpeta de prueba
    let mut init_template = LinkedList::new();
    init_template.push_back(TemplateAttribute::Text { title: "Descripción".to_string() });
    folders.append(Folder {
        name: "General".to_string(),
        template: init_template,
        objects: DynamicArray::new(5),
    });

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
        println!("6. Salir");
        
        match read_input("Selección: ").as_str() {
            "1" => {
                if let Ok(idx) = read_input("Índice de carpeta: ").parse::<usize>() {
                    if let Some(folder) = folders.get_mut(idx) {
                        gestionar_carpeta(folder);
                    } else { println!("Carpeta no encontrada."); }
                }
            }
            "2" => {
                let name = read_input("Nombre de la nueva carpeta: ");
                folders.append(Folder { name, template: LinkedList::new(), objects: DynamicArray::new(5) });
                println!("Carpeta creada.");
            }
            "3" => {
                if let Ok(idx) = read_input("Índice de carpeta: ").parse::<usize>() {
                    if let Some(folder) = folders.get_mut(idx) {
                        folder.name = read_input("Nuevo nombre: ");
                    }
                }
            }
            "4" => {
                if let Ok(from) = read_input("Mover carpeta (índice origen): ").parse::<usize>() {
                    if let Ok(to) = read_input("Hacia (índice destino): ").parse::<usize>() {
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
                        if let Some(removed) = folders.remove(idx) {
                            println!("Carpeta '{}' destruida con todos sus elementos.", removed.name);
                        }
                    }
                }
            }
            "6" => break,
            _ => println!("Opción no válida."),
        }
    }
}

// Submenú para aislar la lógica de gestión de una carpeta específica
fn gestionar_carpeta(folder: &mut Folder) {
    loop {
        println!("\n--- CARPETA: {} ---", folder.name);
        println!("1. GESTIONAR PLANTILLA");
        println!("2. GESTIONAR OBJETOS");
        println!("3. Volver al menú principal");
        
        match read_input("Selección: ").as_str() {
            "1" => {
                loop {
                    println!("\n-- PLANTILLA DE {} --", folder.name);
                    println!("1. Ver Plantilla");
                    println!("2. Agregar Atributo");
                    println!("3. Eliminar Atributo (Cascada)");
                    println!("4. Reorganizar Atributo");
                    println!("5. Volver");
                    
                    match read_input("Opción: ").as_str() {
                        "1" => folder.template.print(),
                        "2" => {
                            println!("\na. Texto | b. Tag Slider | c. Checkbox | d. Single Tag");
                            let attr = match read_input("Tipo: ").as_str() {
                                "a" => TemplateAttribute::Text { title: read_input("Título: ") },
                                "b" => TemplateAttribute::TagSlider { label: read_input("Label: "), min_label: read_input("Min: "), max_label: read_input("Max: ") },
                                "c" => TemplateAttribute::Checkbox { label: read_input("Label: ") },
                                "d" => TemplateAttribute::SingleTag { category: read_input("Categoría: "), color: read_input("Color Hex: ") },
                                _ => continue,
                            };
                            folder.template.push_back(attr);
                            println!("¡Agregado! (Recuerda actualizar los objetos creados previamente)");
                        }
                        "3" => {
                            folder.template.print();
                            if let Ok(idx) = read_input("Índice a eliminar: ").parse::<usize>() {
                                if idx < folder.template.len() {
                                    let confirm = read_input("¿Confirmar eliminación? Se borrará el valor en TODOS los objetos de esta carpeta (S/N): ").to_uppercase();
                                    if confirm == "S" {
                                        if let Some(removed) = folder.template.remove(idx) {
                                            // Borrado en cascada en los objetos
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
                                    if let Some(removed) = folder.template.remove(from) {
                                        folder.template.insert(to, removed);
                                        
                                        // Reorganizar los atributos en cada objeto para que coincidan con la plantilla
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
                        _ => println!("Inválido."),
                    }
                }
            }
            "2" => {
                loop {
                    println!("\n-- OBJETOS DE {} --", folder.name);
                    println!("1. Ver Lista");
                    println!("2. Crear Objeto");
                    println!("3. Editar Objeto");
                    println!("4. Eliminar Objeto");
                    println!("5. Volver");
                    
                    match read_input("Opción: ").as_str() {
                        "1" => {
                            for i in 0..folder.objects.len() {
                                if let Some(obj) = folder.objects.get(i) {
                                    println!("\n[{}] ID: {} | {}", i, obj.id, obj.title);
                                    obj.attributes.print();
                                }
                            }
                        }
                        "2" => {
                            let id = read_input("ID del Objeto: ");
                            let title = read_input("Título: ");
                            let mut obj_values = LinkedList::new();
                            
                            let mut current = &folder.template.head;
                            while let Some(node) = current {
                                match &node.data {
                                    TemplateAttribute::Text { title } => obj_values.push_back(ObjectValue::Text(read_input(&format!("{}: ", title)))),
                                    TemplateAttribute::TagSlider { label, min_label, max_label } => {
                                        let val = loop {
                                            if let Ok(v) = read_input(&format!("{} ({} a {} -> 0-100): ", label, min_label, max_label)).parse::<u8>() {
                                                if v <= 100 { break v; }
                                            }
                                        };
                                        obj_values.push_back(ObjectValue::TagSlider(val));
                                    },
                                    TemplateAttribute::Checkbox { label } => {
                                        let b = loop {
                                            let v = read_input(&format!("{} (S/N): ", label)).to_uppercase();
                                            if v == "S" { break true; } if v == "N" { break false; }
                                        };
                                        obj_values.push_back(ObjectValue::Checkbox(b));
                                    },
                                    TemplateAttribute::SingleTag { category, .. } => obj_values.push_back(ObjectValue::SingleTag(read_input(&format!("Tag para {}: ", category)))),
                                }
                                current = &node.next;
                            }
                            
                            folder.objects.append(InventoryObject { id, title, fallback_images: DynamicArray::new(2), attributes: obj_values });
                            println!("Objeto creado en la carpeta.");
                        }
                        "3" => {
                            if let Ok(idx) = read_input("Índice de objeto a editar: ").parse::<usize>() {
                                if let Some(obj) = folder.objects.get_mut(idx) {
                                    obj.attributes.print();
                                    if let Ok(attr_idx) = read_input("Índice de atributo: ").parse::<usize>() {
                                        if let Some(val_ref) = obj.attributes.get_mut(attr_idx) {
                                            match val_ref {
                                                ObjectValue::Text(s) => *s = read_input("Nuevo Texto: "),
                                                ObjectValue::TagSlider(num) => *num = read_input("Nuevo valor (0-100): ").parse().unwrap_or(*num),
                                                ObjectValue::Checkbox(b) => *b = !*b,
                                                ObjectValue::SingleTag(t) => *t = read_input("Nuevo Tag: "),
                                            }
                                            println!("Editado.");
                                        }
                                    }
                                }
                            }
                        }
                        "4" => {
                            if let Ok(idx) = read_input("Índice de objeto a eliminar: ").parse::<usize>() {
                                if let Some(removed) = folder.objects.remove(idx) {
                                    println!("Objeto {} eliminado.", removed.title);
                                }
                            }
                        }
                        "5" => break,
                        _ => println!("Inválido."),
                    }
                }
            }
            "3" => break,
            _ => println!("Opción no válida."),
        }
    }
}