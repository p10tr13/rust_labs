use std::cell::{Cell, OnceCell, LazyCell, RefCell};
use std::rc::{Rc, Weak};
use std::ops::{Deref, DerefMut};
use std::borrow::Cow;
use std::collections::VecDeque;
use std::{fs, io};
use std::path::{Path, PathBuf};

struct AustroHungarianGreeter {
    index: Cell<usize>,
    n: u32
}

impl AustroHungarianGreeter {
    fn new() -> AustroHungarianGreeter {
        AustroHungarianGreeter {
            index: Cell::new(0),
            n: 0
        }
    }

    fn greet(&mut self) -> &'static str {
        const MESSAGES: [&str; 3] = [
            "Es lebe der Kaiser!",
            "Möge uns der Kaiser schützen!",
            "Éljen Ferenc József császár!",
        ];

        let current_index = self.index.get();
        let message = MESSAGES[current_index];
        self.index.set((current_index + 1) % MESSAGES.len());
        self.n += 1;
        message
    }
}

impl Drop for AustroHungarianGreeter {
    fn drop(&mut self) {
        println!("Ich habe {} mal gegrüßt", self.n);
    }
}

pub enum HeapOrStack<T> {
    Stack(T),
    Heap(Box<T>)
}

impl<T> Deref for HeapOrStack<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            HeapOrStack::Stack(v) => v,
            HeapOrStack::Heap(b) => b,
        }
    }
}

impl<T> DerefMut for HeapOrStack<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            HeapOrStack::Stack(v) => v,
            HeapOrStack::Heap(b) => b,
        }
    }
}

pub fn canon_head<'a>(xs: &'a VecDeque<i32>)
    -> Option<Cow<'a, VecDeque<i32>>> {
    if xs.is_empty() {
        return Some(Cow::Borrowed(xs));
    }

    let ind = xs.iter().position(|&x| x % 2 == 1)?;

    if ind == 0 {
        return Some(Cow::Borrowed(xs));
    }

    let mut owned = xs.clone();
    owned.rotate_left(ind);
    Some(Cow::Owned(owned))
}

struct CachedFile {
    cache: OnceCell<String>
}

impl CachedFile {
    fn new() -> Self {
        Self { cache: OnceCell::new() }
    }

    pub fn get(&self, path: &Path) -> Result<&str, io::Error> {
        if let Some(content) = self.cache.get() {
            return Ok(content);
        }

        let loaded_content = fs::read_to_string(path)?;

        let _ = self.cache.set(loaded_content);

        Ok(self.cache.get().unwrap())
    }

    pub fn try_get(&self) -> Option<&str> {
        self.cache.get().map(|s| s.as_str())
    }
}

#[derive(Clone)]
pub struct SharedFile {
    file: Rc<LazyCell<String, Box<dyn FnOnce() -> String>>>,
}

impl SharedFile {
    pub fn new(path: PathBuf) -> Self {
        let initializer = Box::new(move || {
            println!("Trying to read a file in SharedFile.");
            fs::read_to_string(&path).unwrap_or_else(|_| {
                format!("Error reading file: {:?}", path)
            })
        });
        Self {
            file: Rc::new(LazyCell::new(initializer))
        }
    }

    pub fn get(&self) -> &str {
        &self.file
    }
}

pub struct Vertex {
    pub out_edges_owned: Vec<Rc<RefCell<Vertex>>>,
    pub out_edges: Vec<Weak<RefCell<Vertex>>>,
    pub data: i32
}

impl Vertex {
    pub fn new() -> Self {
        Vertex {
            out_edges_owned: Vec::new(),
            out_edges: Vec::new(),
            data: 0
        }
    }

    pub fn create_neighbour(&mut self) -> Rc<RefCell<Vertex>> {
        let new_vertex = Rc::new(RefCell::new(Vertex::new()));
        self.out_edges_owned.push(new_vertex.clone());
        new_vertex
    }

    pub fn link_to(&mut self, other: &Rc<RefCell<Vertex>>) {
        let weak_ref = Rc::downgrade(other);
        self.out_edges.push(weak_ref);
    }

    pub fn all_neighbours(&self) -> Vec<Weak<RefCell<Vertex>>> {
        let mut all_neighbours = Vec::new();
        for owned in &self.out_edges_owned {
            all_neighbours.push(Rc::downgrade(owned));
        }
        for weak in &self.out_edges {
            all_neighbours.push(weak.clone());
        }
        all_neighbours
    }

    pub fn cycle(n: usize) -> Rc<RefCell<Vertex>> {
        if n == 0 {
            return Rc::new(RefCell::new(Vertex::new()));
        }
        let head = Rc::new(RefCell::new(Vertex::new()));
        head.borrow_mut().data = 0;

        let mut current = head.clone();

        for i in 1..n {
            let next = current.borrow_mut().create_neighbour();
            next.borrow_mut().data = i as i32;
            current = next;
        }
        current.borrow_mut().link_to(&head);

        head
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    // Exercise 1-2
    let mut ahg = AustroHungarianGreeter::new();
    for _ in 0..4 {
        println!("{}", ahg.greet());
    }

    // Exercise 3
    let mut a = HeapOrStack::Stack(10);
    let mut b = HeapOrStack::Heap(Box::new(20));
    println!("a = {}, b = {}", *a, *b);
    *a += 5;
    *b += 7;
    println!("a = {}, b = {}", *a, *b);

    // Exercise 5
    let file_cache = CachedFile::new();
    if let Some(val) = file_cache.try_get() {
        println!("This won't print, cuz cache is empty: {}", val);
    } else {
        println!("Cache is empty.");
    }
    let path = Path::new("non_existent_file.txt");
    match file_cache.get(path) {
        Ok(text) => println!("File read: {}", text),
        Err(e) => eprintln!("Error reading: {}", e),
    }

    // Exercise 6
    let path = PathBuf::from("text_file.txt");
    let _ = fs::write(&path, "Shared test data");
    let file_ref1 = SharedFile::new(path.clone());
    let file_ref2 = file_ref1.clone();
    println!("Refs created, but file not read yet.");
    println!("Content (ref2): {}", file_ref2.get());
    println!("Content (ref1): {}", file_ref1.get());
    let _ = fs::remove_file(path);

    // Exercise 7
    let cycle_length = 3;
    println!("Creating cycle with length: {}", cycle_length);
    let cycle_head = Vertex::cycle(cycle_length);
    let neighbours = cycle_head.borrow().all_neighbours();
    if let Some(first_weak) = neighbours.first()
        && let Some(v1_rc) = first_weak.upgrade() {
        println!("Neighbour of cycle head is: {}, strong_count of this vertex is: {}",
                 v1_rc.borrow().data, Rc::strong_count(&v1_rc));
        let v1_neighbours = v1_rc.borrow().all_neighbours();
        if let Some(v2_weak) = v1_neighbours.first()
            && let Some(v2_rc) = v2_weak.upgrade() {
            println!("Neighbour of v1 is v{}", v2_rc.borrow().data);
            let v2_neighbours = v2_rc.borrow().all_neighbours();
            if let Some(v0_back_weak) = v2_neighbours.first()
                && let Some(v0_back_rc) = v0_back_weak.upgrade() {
                println!("Neighbour of v2 is v{} (Again beginning!)", v0_back_rc.borrow().data);
            }
        }
    }
}
