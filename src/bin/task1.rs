fn main() {
    // Leeren Vektor erstellen
    let mut numbers: Vec<i32> = Vec::new();

    // Zahlen hinzufügen
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    // Alle Elemente ausgeben
    println!("Die Elemente im Vektor sind:");
    for number in &numbers {
        println!("{}", number);
    }

    // Summe berechnen und ausgeben
    let sum: i32 = numbers.iter().sum();
    println!("Die Summe der Elemente ist: {}", sum);
}
