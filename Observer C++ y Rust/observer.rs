trait Observer {
        fn update(&self);
}

// Guarda en memoria los observadores y los notifica para actualizar
trait Observable<'a, T: Observer> {
fn Añadir_observador(&mut self, observer: &'a T);
fn Eliminar_observador(&mut self, observer: &'a T);
fn Notifica_observador(&self);
}


struct Display {
    name: String,
}
struct Clima<'a, T:'a> {
    temperature: f64,
            observers: Vec<&'a T>
}
        impl<'a> Clima<'a, Display> {
        fn set_temperature(&mut self, temperature: f64) {
            self.temperature = temperature;
            let temp: String;
            temp = temperature.to_string();
            println!("La temperatura cambio a: {}",temp);
            self.notify_observers();

        }

//impl<U: Observable> Observer<U> for Display<U> {
impl Observer for Display {
fn update(&self) {
    println!("Observador actualizado", self.name);
}
}
impl Display {
        fn new(name: String) -> Display {
            Display{name: name}
        }
}
impl std::cmp::PartialEq for Display {
fn eq(&self, other: &Display) -> bool {
self.name == other.name
}
}
impl std::fmt::Display for Display {
fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
write!(f, "Display {}", self.name)
}
}

impl<'a, T: Observer+PartialEq+std::fmt::Display> Observable<'a, T> for Clima<'a, T> {
fn add_observer(&mut self, observer: &'a T) {
println!("Añadir_observador({});", observer);
self.observers.push(observer);
}
fn Eliminar_observador(&mut self, observer: &'a T) {
let mut index = 0;
let mut found = false;
for &obs in self.observers.iter() {
    if obs == observer {
                println!("Eliminar observador({});", observer);
                found = true;
                break;
        }
    index += 1;
}
if found {
self.observers.remove(index);
}
}
fn Notifica_observador(&self) {
    for &observer in self.observers.iter() {
        observer.update();
    }
}
}

fn main() {
    let observador = Display::new("Observador".to_string());
    let observador2 = Display::new("Observador2".to_string());
    let mut clima = Clima{temperature: 19.0, observers: Vec::new()};
    clima.add_observer(&observador);
    clima.add_observer(&observador2);
    clima.set_temperature(20.0);
    clima.delete_observer(&observador2);
    clima.set_temperature(21.0);
}
