import { faker } from '@faker-js/faker';
import fs from 'fs';

// Lista de climas posibles
const weathers = ['Lluvioso', 'Nubloso', 'Soleado'];

// Arreglo para almacenar los objetos generados
let weatherData = [];

for (let i = 0; i < 10000; i++) {
    // Generar el clima de forma aleatoria
    const weather = weathers[Math.floor(Math.random() * weathers.length)];

    // Generar la descripción basada en el clima
    let description;
    switch (weather) {
        case 'Lluvioso':
            description = "Está lloviendo";
            break;
        case 'Nubloso':
            description = "Está nubloso";
            break;
        case 'Soleado':
            description = "Está soleado";
            break;
        default:
            description = "Estado del clima desconocido";
            break;
    }

    // Generar un país de forma aleatoria (usando un código de país)
    const country = faker.location.countryCode();

    // Crear el objeto y agregarlo al arreglo
    weatherData.push({
        description: description,
        country: country,
        weather: weather
    });
}

fs.writeFileSync('../generated/weatherData.json', JSON.stringify(weatherData, null, 4));