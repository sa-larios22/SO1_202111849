use std::process::Command;

fn main() {
    println!("Ejecutando el contenedor de Docker...");
    
    // Ejecutar el contenedor con Docker Compose
    let status = Command::new("docker")
        .arg("compose")
        .arg("up")
        .arg("--build")
        .status()
        .expect("Error al ejecutar Docker Compose");

    if !status.success() {
        eprintln!("Hubo un error al ejecutar Docker Compose");
    }
}
