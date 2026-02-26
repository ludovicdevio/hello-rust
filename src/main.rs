
fn main() {
    

    println!("Bonjour, le monde!");
    println!("Hello, world!");

    // Variables et types de données
    let nom = "Ludo";
    let age = 12;
    let taille = 1.80;
    let cto = false;
    let autre = "E";
    println!("Bonjour, {}!", nom);
    print!("Tu as {} ans, tu mesures {} mètres, et tu es CTO: {}. Ton autre est {}.", age, taille, cto, autre);

    //Constantes
    const PI: f64 = 3.14159;
    const VITESSE_LUMIERE: u32 = 299_792_458; // en mètres par seconde
    const ANNEE_EN_COURS: i32 = 2026;
    const NOM: &str = "Ludo";
    const PLANETE: &str = "Terre";
    const INGREDIENTS: [&str; 5] = ["Farine", "Sucre", "Beurre", "Oeufs", "Vanille"];

    println!("\nLa valeur de PI est: {}", PI);
    println!("La vitesse de la lumière est: {} m/s", VITESSE_LUMIERE);
    println!("L'année en cours est: {}", ANNEE_EN_COURS);
    println!("Mon nom est: {}", NOM);
    println!("Je vis sur la planète: {}", PLANETE);
    println!("Les ingrédients pour la recette sont: {:?}", INGREDIENTS);

    // Opérateurs arithmétiques
    let a = 10;
    let b = 5;
    let somme = a + b;
    let difference = a - b;
    let produit = a * b;
    let quotient = a / b;
    let reste = a % b;

    println!("Opérations sur {} et {}: somme={}, différence={}, produit={}, quotient={}, reste={}", a, b, somme, difference, produit, quotient, reste);

    // Opérateurs d'affectation
    let mut x = 10;
    x += 5; // x = x + 5
    println!("Après x += 5, x = {}", x);
    x -= 3; // x = x - 3
    println!("Après x -= 3, x = {}", x);
    x *= 2; // x = x * 2
    println!("Après x *= 2, x = {}", x);
    x /= 4; // x = x / 4
    println!("Après x /= 4, x = {}", x);
    x %= 3; // x = x % 3
    println!("Après x %= 3, x = {}", x);

    // Opérateurs de comparaison
    let p = 10;
    let q = 20;
    println!("p == q: {}", p == q);
    println!("p != q: {}", p != q);
    println!("p > q: {}", p > q);
    println!("p < q: {}", p < q);
    println!("p >= q: {}", p >= q);
    println!("p <= q: {}", p <= q); 
    println!("\n");

    // Opérateurs logiques
    let r = true;
    let s = false;
    println!("r && s: {}", r && s);
    println!("r || s: {}", r || s);
    println!("!r: {}", !r);


}