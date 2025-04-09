#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    /// Erstellt einen neuen Punkt.
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    /// Gibt die Koordinaten des Punkts zurück.
    fn get(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    /// Setzt die Koordinaten des Punkts.
    fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    /// Verschiebt den Punkt um die angegebenen Werte.
    fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }

    /// Berechnet die Entfernung zu einem anderen Punkt.
    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    /// Berechnet die Entfernung zum Ursprung.
    fn abs(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let mut p1 = Point::new(3.0, 4.0);
    println!("Punkt 1: {:?}", p1);

    let p2 = Point::new(6.0, 8.0);
    println!("Punkt 2: {:?}", p2);

    println!("Entfernung zwischen P1 und P2: {}", p1.distance(&p2));
    
    p1.translate(1.0, 1.0);
    println!("Punkt 1 nach Translation: {:?}", p1);

    println!("Entfernung von P1 zum Ursprung: {}", p1.abs());
}
