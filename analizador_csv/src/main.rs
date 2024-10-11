use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use csv::{ReaderBuilder, WriterBuilder};

#[derive(Debug, Deserialize)]
struct Register {
    #[serde(rename = "Songs & Artist")]
    artist_song: String,
    #[serde(rename = "Streams")]
    streams: Option<u64>,
    #[serde(rename = "Daily")]
    daily: Option<u64>,
}

// Structure to store information grouped by artist, now with Serialize for export
#[derive(Debug, Default, Serialize)]
struct ArtistData {
    artist: String,
    total_streams: u64,
    total_daily: u64,
    max_streams: u64,
    song_count: u64,
    average_streams: f64,
    average_daily: f64,
}

/// Reads data from a CSV file and returns a vector of records
fn read_csv(file_route: &str) -> Result<Vec<Register>, Box<dyn Error>> {
    let csv_file = File::open(file_route)?; // Open the file
    let mut reader = ReaderBuilder::new().from_reader(csv_file); // Create a CSV reader
    let mut registers: Vec<Register> = Vec::new();

    // Process each record in the file
    for result in reader.deserialize() {
        let register: Register = result?; // Deserialize each row to the Register structure
        registers.push(register); // Add the record to the vector
    }
    Ok(registers)
}

/// Processes records to group and calculate statistics by artist
fn process_artist_data(registers: &[Register]) -> HashMap<String, ArtistData> {
    let mut artist_data_map: HashMap<String, ArtistData> = HashMap::new();

    for register in registers {
        // Separate the artist name and the song name
        let parts: Vec<&str> = register.artist_song.split(" - ").collect();
        if parts.len() != 2 {
            continue; // Skip malformed records
        }
        let artist = parts[0].to_string();

        // Obtiene o crea una entrada para el artista
        let data = artist_data_map.entry(artist.clone()).or_insert_with(ArtistData::default);

        // Update stream data and song count
        if let Some(streams) = register.streams {
            data.total_streams += streams;
            data.max_streams = data.max_streams.max(streams);
        }
        if let Some(daily) = register.daily {
            data.total_daily += daily;
        }
        data.song_count += 1;
        data.artist = artist.clone();
    }

    // Calculate averages
    for data in artist_data_map.values_mut() {
        if data.song_count > 0 {
            data.average_streams = data.total_streams as f64 / data.song_count as f64;
            data.average_daily = data.total_daily as f64 / data.song_count as f64;
        }
    }

    artist_data_map
}

/// Export the data summary by artist to a CSV file
fn write_artist_data(file_route: &str, artist_data_map: &HashMap<String, ArtistData>) -> Result<(), Box<dyn Error>> {
    let file = File::create(file_route)?; // Create the output file
    let mut writer = WriterBuilder::new().from_writer(file); // Initialize a CSV writer

    // Write each ArtistData entry to the file
    for data in artist_data_map.values() {
        writer.serialize(data)?; // Serialize each ArtistData and write it to the file
    }
    writer.flush()?; // Ensures that all data has been written
    Ok(())
}

fn main() {
    let input_file = "spotify_data.csv"; // Input file path
    let output_file = "artist_data_summary.csv"; // Output file path

    // Read the CSV file and process the data
    match read_csv(input_file) {
        Ok(registers) => {
            let artist_data_map = process_artist_data(&registers);

            // Export the data grouped by artist to a new CSV file
            match write_artist_data(output_file, &artist_data_map) {
                Ok(_) => println!("Datos de artista exportados exitosamente a {}", output_file),
                Err(e) => println!("Error al escribir el archivo de salida: {}", e),
            }
        }
        Err(e) => println!("Error al leer el archivo CSV: {}", e),
    }
}
