use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Lenovo IdeaPad Laptop".to_string(),
            price: 799.99,
            description: "A reliable laptop for everyday work, online classes, and entertainment with fast performance and a sleek design.".to_string(),
            image: "/laptop.jpg".to_string()
        },
        Product {
            id: 2,
            name: "Sony Wireless Headphones".to_string(),
            price: 149.99,
            description: "Comfortable wireless headphones with clear sound quality and long battery life for music, calls, and travel.".to_string(),
            image: "/headphones.jpg".to_string()
        },
        Product {
            id: 3,
            name: "Samsung Galaxy Smartphone".to_string(),
            price: 999.99,
            description: "A powerful smartphone with a high-resolution display, advanced camera system, and all-day battery life.".to_string(),
            image: "/smartphone.jpg".to_string()
        },
        Product {
            id: 4,
            name: "LG 27-inch 4K Monitor".to_string(),
            price: 329.99,
            description: "A sharp 4K monitor ideal for productivity, streaming, and creative work with vivid color and detail.".to_string(),
            image: "/monitor.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Logitech Mechanical Keyboard".to_string(),
            price: 119.99,
            description: "A responsive mechanical keyboard designed for comfortable typing and gaming with durable key switches.".to_string(),
            image: "/keyboard.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Apple iPad Air".to_string(),
            price: 729.99,
            description: "A lightweight tablet perfect for studying, note-taking, browsing, and entertainment with smooth performance.".to_string(),
            image: "/tablet.jpg".to_string()
        },
        Product {
            id: 7,
            name: "Canon EOS Camera".to_string(),
            price: 649.99,
            description: "A versatile digital camera for capturing high-quality photos and videos with easy-to-use controls.".to_string(),
            image: "/camera.jpg".to_string()
        },
        Product {
            id: 8,
            name: "JBL Bluetooth Speaker".to_string(),
            price: 89.99,
            description: "A portable Bluetooth speaker with rich sound, compact design, and reliable battery life for music anywhere.".to_string(),
            image: "/speaker.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Xbox Wireless Controller".to_string(),
            price: 74.99,
            description: "A comfortable wireless controller with precise input and ergonomic grip for console and PC gaming.".to_string(),
            image: "/controller.jpg".to_string()
        },
        Product {
            id: 10,
            name: "HP All-in-One Printer".to_string(),
            price: 159.99,
            description: "An all-in-one printer for printing, scanning, and copying documents at home or in a small office.".to_string(),
            image: "/printer.jpg".to_string()
        }
    ]
}