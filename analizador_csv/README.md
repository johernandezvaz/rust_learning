# CSV Data Analyzer in Rust

## Project Overview

This Rust program analyzes a CSV file (`spotify_data.csv`) containing data on songs, streams, and daily play counts. The main objective of the project is to provide statistical insights into the popularity of various artists by calculating metrics such as the total, average, and maximum streams per artist.

The processed data is then saved in a new CSV file called `artist_data_summary.csv`, which can be opened in any CSV viewer or loaded into a Jupyter notebook for further exploration.

## Project Objectives

1. **CSV Data Reading**: Parse the original `spotify_data.csv` file to read song and artist data, total streams, and daily streams.
2. **Data Aggregation**: Separate artist names from song titles and aggregate statistics per artist.
3. **Statistical Calculations**:
   - Calculate the total streams for each artist.
   - Calculate the daily play counts for each artist.
   - Find the maximum streams for each artist.
   - Calculate averages for both streams and daily play counts per artist.
4. **Data Export**: Save the aggregated data to a new CSV file (`artist_data_summary.csv`) that can be used for analysis or visualized in a Jupyter notebook.

## Code Explanation

### Key Code Components

1. **Struct Definitions**:
   - `Register`: Represents each row in the input CSV, containing `artist_song`, `streams`, and `daily` fields.
   - `ArtistData`: Stores aggregated data for each artist, including totals, averages, and maximum values for streams and daily play counts.

2. **Function Descriptions**:
   - `read_csv`: Reads and parses the CSV file, returning a list of `Register` instances for each row.
   - `process_artist_data`: Processes and aggregates data for each artist, calculates totals, averages, and maximum values, and updates `ArtistData` entries.
   - `write_artist_data`: Exports the `ArtistData` entries to `artist_data_summary.csv`, creating a CSV file with all calculated statistics.

3. **Main Function**:
   - Reads the CSV data and aggregates it by artist.
   - Outputs the processed data to `artist_data_summary.csv`.

### Code Flow

1. The program reads the `spotify_data.csv` file and extracts the data row by row.
2. Each artist is separated from their song title in the `artist_song` column.
3. The program then calculates total streams, daily counts, maximum streams, and averages for each artist.
4. Finally, all artist statistics are written to the new `artist_data_summary.csv` file.

## Viewing the Data in Jupyter Notebook

To further explore and visualize the processed data:

1. Open a Jupyter notebook in the same directory as `artist_data_summary.csv`.
2. Use the following code to load and view the CSV data:

   ```python
   import pandas as pd

   # Load processed artist data
   df = pd.read_csv("artist_data_summary.csv")

   # Display data
   df.head()
