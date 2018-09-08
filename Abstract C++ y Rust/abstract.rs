
/*
 * Define celular
 */
trait celular {
        fn call(&self);
}

/*
 * Define consola
 */
trait consola {
        fn play_games(&self);
}

/*
 * Define fabrica
 */
trait Factory<P: celular, T: consola> {
fn new_celular(&self) -> P;
fn new_consola(&self) -> T;
}


/*
 * Define productos Sony
 */
struct CelularSony;

impl celular for CelularSony {
fn call(&self) {
    println!("Look! I'm calling on an CelularSony!");
}
}

struct Playstation;

impl consola for Playstation {
fn play_games(&self) {
    println!("Just playing some games on my Playstation.");
}
}

/*
 * Crea fábrica Sony
 */
struct fábricaSony;

impl Factory<CelularSony, Playstation> for fábricaSony {
fn new_celular(&self) -> CelularSony {
return CelularSony;
}

fn new_consola(&self) -> Playstation {
return Playstation;
}
}

/*
 * Define productos Microsoft
 */

struct CelularMicrosoft;

impl celular for CelularMicrosoft {
fn call(&self) {
    println!("Look! I'm calling on a CelularMicrosoft!");
}
}

struct Xbox;

impl consola for Xbox {
fn play_games(&self) {
    println!("Just playing some games on my Xbox");
}
}

/*
 * Crea fábrica Microsoft
 */
struct fábricaMicrosoft;

impl Factory<CelularMicrosoft, Xbox> for fábricaMicrosoft {
fn new_celular(&self) -> CelularMicrosoft {
return CelularMicrosoft;
}

fn new_consola(&self) -> Xbox {
return Xbox;
}
}


fn main() {
    // Create our two different factories
    let sony = AppleFactory;
    let microsoft = GoogleFactory;


    // Test out creating phones
    let celular = sony.new_celular();
    celular.call();

    let celular = microsoft.new_celular();
    celular.call();

    // Test out creating tablets
    let consola = sony.new_consola();
    consola.play_games();

    let consola = microsoft.new_consola();
    consola.play_games();
}
