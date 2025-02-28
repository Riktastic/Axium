### Setup Instructions for Axium within Docker

This guide walks you through building **Axium** as a docker container.

Make sure that you have docker aswell as docker compose (which is bundled with docker in the most recent releases of docker) installed.

Please note! That in a production system I wouldn't want to run the database on the same server as the API.

The shown commands except nano will work on Windows' PowerShell (you can use notepad.exe instead of nano).

---

1. **Clone the Axium repository**:
   Clone the Axium repository from GitHub:
   ```sh
   git clone https://github.com/Riktastic/Axium.git
   ```

---

2. **Navigate to the Axium directory**:
   Move into the cloned repository:
   ```sh
   cd Axium
   ```

---

3. **Copy the Example environment file**:
   Copy the `.env.example` file to `.env` to configure your environment:
   ```sh
   cp .env.example .env
   ```

---

4. **Edit the `.env` File**:
   Update the `.env` file with your database settings if necessary:
   ```sh
   nano .env
   ```

---

1. **Build and run the container**:
   Start Axium using Docker Compose:
   ```sh
   docker compose up
   ```

---

After sucessfully building the image, it will start two containers:
- axium-axium-1: Axium,
- axium-db-1: The PostgreSQL database.

The database will store its files within the `./docker` folder.
