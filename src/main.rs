use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use colored::Colorize;
use clap::Arg;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ServiceInfo {
    name: String,
    display_name: String,
    status: String,
}

impl ServiceInfo {
    fn new(name: &str, display_name: &str, status: &str) -> Self {
        ServiceInfo {
            name: name.to_string(),
            display_name: display_name.to_string(),
            status: status.to_string(),
        }
    }
}

fn main() -> io::Result<()> {
    let services = vec![
        ServiceInfo::new("Service1", "Service 1", "CONTINUE_PENDING"),
        ServiceInfo::new("Service2", "Service 2", "PAUSED"),
        ServiceInfo::new("Service3", "Service 3", "PAUSE_PENDING"),
        ServiceInfo::new("Service4", "Service 4", "RUNNING"),
        ServiceInfo::new("Service5", "Service 5", "START_PENDING"),
        ServiceInfo::new("Service6", "Service 6", "STOP_PENDING"),
        ServiceInfo::new("Service7", "Service 7", "STOPPED"),
        ServiceInfo::new("Service8", "Service 8", "RUNNING"),
        ServiceInfo::new("Service9", "Service 9", "RUNNING"),
    ];

    loop {
        println!("\nSeleccione una opción:");
        println!("1. Open Service");
        println!("2. Close Service");
        println!("3. List Services");
        print!("Ingrese el número de la opción deseada: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        match input {
            "1" | "Open Service" => open_service_menu(&services)?,
            "2" | "Close Service" => close_service_menu(&services)?,
            "3" | "List Services" => list_services(&services)?,
            _ => println!("Opción no válida. Intente nuevamente."),
        }

        println!("\n¿Desea realizar otra acción? (s/n)");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim().to_lowercase();
        if choice != "s" {
            break;
        }
    }

    Ok(())
}

fn open_service_menu(services: &[ServiceInfo]) -> io::Result<()> {
    let menu_items: Vec<&str> = services
        .iter()
        .filter(|s| !["RUNNING", "CONTINUE_PENDING", "START_PENDING"].contains(&s.status.as_str()))
        .map(|s| s.name.as_str())
        .collect();
    
    if menu_items.is_empty() {
        println!("No hay servicios disponibles para abrir.");
        return Ok(());
    }

    println!("Selecciona un servicio para abrir:");
    for (index, item) in menu_items.iter().enumerate() {
        println!("{}: {}", index + 1, item);
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let index: usize = match input.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= menu_items.len() => num - 1,
        _ => {
            println!("Selección inválida.");
            return Ok(());
        }
    };

    let selected_service = menu_items[index];
    println!("Abriendo el servicio: {}", selected_service);
    // Aquí se implementaría la lógica para abrir el servicio seleccionado.
    Ok(())
}

fn close_service_menu(services: &[ServiceInfo]) -> io::Result<()> {
    let menu_items: Vec<&str> = services
        .iter()
        .filter(|s| s.status == "RUNNING")
        .map(|s| s.name.as_str())
        .collect();
    
    if menu_items.is_empty() {
        println!("No hay servicios disponibles para cerrar.");
        return Ok(());
    }

    println!("Selecciona un servicio para cerrar:");
    for (index, item) in menu_items.iter().enumerate() {
        println!("{}: {}", index + 1, item);
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let index: usize = match input.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= menu_items.len() => num - 1,
        _ => {
            println!("Selección inválida.");
            return Ok(());
        }
    };

    let selected_service = menu_items[index];
    println!("Cerrando el servicio: {}", selected_service);
    // Aquí se implementaría la lógica para cerrar el servicio seleccionado.
    Ok(())
}

fn list_services(services: &[ServiceInfo]) -> io::Result<()> {
    println!("{}", "Lista de Servicios");
    for service in services {
        println!(
            "{}: {} | Status: {}",
            service.name,
            service.display_name,
            match service.status.as_str() {
                "CONTINUE_PENDING" => service.status.clone().on_green().to_string(),
                "PAUSED" => service.status.clone().yellow().to_string(),
                "PAUSE_PENDING" => service.status.clone().on_yellow().to_string(),
                "RUNNING" => service.status.clone().green().to_string(),
                "START_PENDING" => service.status.clone().on_green().to_string(),
                "STOP_PENDING" => service.status.clone().on_red().to_string(),
                "STOPPED" => service.status.clone().bright_red().to_string(),
                _ => service.status.clone().normal().to_string(),
            }
        );
    }

    Ok(())
}
