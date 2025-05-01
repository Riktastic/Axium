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

Axium can send emails for actions like user registration, password reset, etc. To configure the SMTP settings:

4. **Edit the `.env` file**:
   Copy the `.env.example` file to ``.env`.
   Open your `.env` file and update the following mail-related environment variables:
   ```sh
   nano .env
   ```

   Update the SMTP configuration with your provider's details:
   ```env
   MAIL_SERVER="smtp.example.com"  # Replace with your SMTP server
   MAIL_PORT="587"  # Usually 587 for STARTTLS, 465 for SSL/TLS
   MAIL_USER="your@email.com"  # Your email address or username
   MAIL_PASS="your_smtp_password"  # Your SMTP password
   ```

   **Common Providers**:
   - For **Gmail**, use:
     ```env
     MAIL_SERVER="smtp.gmail.com"
     MAIL_PORT="587"
     MAIL_USER="your.email@gmail.com"
     MAIL_PASS="your_gmail_password_or_app_password"
     ```

   - For **SendGrid**, use:
     ```env
     MAIL_SERVER="smtp.sendgrid.net"
     MAIL_PORT="587"
     MAIL_USER="apikey"
     MAIL_PASS="your_sendgrid_api_key"
     ```

   - For **Mailtrap** (for testing), use:
     ```env
     MAIL_SERVER="smtp.mailtrap.io"
     MAIL_PORT="587"
     MAIL_USER="your_mailtrap_username"
     MAIL_PASS="your_mailtrap_password"
     ```

---

5. **Build and run the container**:
   Start Axium using Docker Compose:
   ```sh
   docker compose up
   ```

---

After sucessfully building the image, it will start two containers:
- axium-axium-1: Axium,
- axium-db-1: The PostgreSQL database,
- axium-cache-1: The Redis cache instance,
- axium-storage-1: The MinIO storage instance.

The database, cache, storage databe will be stored within the `./docker/data` folder.

If everything started up sucessfully you should be able to visit Axium using `http://127.0.0.1:8000`.
