
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

    //Strucrures conditionnelles
    let nombre = 15;
    if nombre < 10 {
        println!("Le nombre est inférieur à 10.");
    } else if nombre == 10 {
        println!("Le nombre est égal à 10.");
    } else {
        println!("Le nombre est supérieur à 10.");  
    }
    println!("\n");

    // Boucles
    println!("Boucle for:");
    for i in 0..5 {

        println!("i = {}", i);
    }

    println!("Boucle while:");
    let mut j = 0;
    while j < 5 {
        println!("j = {}", j);
        j += 1;
    }

    print!("Boucle loop:");
    let mut k = 0;
    loop {
        if k >= 5 {
            break;
        }
        println!("k = {}", k);
        k += 1;
    }

    println!("\n");

    // Fonctions
    fn addition(a: i32, b: i32) -> i32 {
        a + b
    }
    let resultat = addition(5, 3);
    println!("Le résultat de l'addition est: {}", resultat);

    //Propriétés de la mémoire
    let mut v = vec![1, 2, 3];
    println!("Le vecteur v est: {:?}", v);
    v.push(4);
    println!("Après avoir ajouté 4, le vecteur v est: {:?}", v);

    println!("\n");

    let w: String = String::from("Hello");
    let z = w; // w est déplacé vers z, w n'est plus valide
    println!("La chaîne z est: {}", z);

    println!("\n");

    let t = 5;
    let m=t;
    println!("t = {}, m = {}", t, m); // t et m sont tous les deux valides
    println!("\n");

    let s1 = String::from("Salut le monde");
    let s2 = s1.clone(); // s1 est cloné vers s2, s1 et s2 sont tous les deux valides
    println!("s1 = {}, s2 = {}", s1, s2);

    println!("\n");

    //References et emprunts
    let u = String::from("Bonjour");
    let v = &u; // v est une référence à u, u reste valide
    println!("u = {}, v = {}", u, v);

    fn afficher_longueur(s: &String) {
        println!("La longueur de '{}' est {}", s, s.len());
    }
    let s3 = String::from("Emprunt en Rust");
    afficher_longueur(&s3); // on passe une référence à s3

    println!("\n");

    // Structures de données
        // Structs
    struct Personne {
        nom: String,
        age: u32,
    }
    let personne1 = Personne {
        nom: String::from("Alice"),
        age: 30,
    };
    println!("La personne s'appelle {} et a {} ans.", personne1.nom, personne1.age);
    println!("\n");

        // Enums
    enum Couleur {
        Rouge,
        Vert,
        Bleu,
    }
    let couleur_preferee = Couleur::Vert;
    match couleur_preferee {
        Couleur::Rouge => println!("La couleur préférée est rouge."),
        Couleur::Vert => println!("La couleur préférée est verte."),
        Couleur::Bleu => println!("La couleur préférée est bleue."),
    }
    println!("\n");

        //Arrays et slices
    let tableau: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Le tableau est: {:?}", tableau);
    let tranche: &[i32] = &tableau[1..4]; // une tranche du tableau
    println!("La tranche du tableau est: {:?}", tranche);

    println!("\n");

        //Collections
    let mut vecteur: Vec<String> = Vec::new();
    vecteur.push(String::from("Rust"));
    vecteur.push(String::from("Langage"));
    println!("Le vecteur est: {:?}", vecteur);
    println!("\n");

        //Vecteurs
    let mut nombres: Vec<i32> = vec![1, 2, 3];
    nombres.push(4);
    println!("Le vecteur de nombres est: {:?}", nombres);   
    println!("\n");

        //Tuples
    let tup: (i32, f64, &str) = (42, 3.14, "Hello");
    println!("Le tuple est: {:?}", tup);
    println!("\n");

        //HashMaps
    use std::collections::HashMap;
    let mut capitales: HashMap<&str, &str> = HashMap::new();
    capitales.insert("France", "Paris");
    capitales.insert("Espagne", "Madrid");
    capitales.insert("Italie", "Rome");
    println!("Les capitales sont: {:?}", capitales);

    //Gestion des erreurs
    fn division(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("Erreur: division par zéro"))
        } else {
            Ok(a / b)
        }
    }
    match division(10.0, 0.0) {
        Ok(resultat) => println!("Le résultat de la division est: {}", resultat),
        Err(e) => println!("{}", e),
    }

    
}