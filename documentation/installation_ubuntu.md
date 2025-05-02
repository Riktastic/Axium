# **Setup Instructions for Axium on Ubuntu 24.04.01 LTS**

This guide walks you through setting up the **Axium** project on an Ubuntu system. It includes the installation of necessary tools like Rust, SQLx, PostgreSQL, Redis, MinIO, and Mail (SMTP). 

Please note: In a production environment, it's generally not recommended to run the database on the same server as the API.

---

### **1. Update the System**
```sh
sudo apt update && sudo apt upgrade -y
```

---

### **2. Install Rust using RustUp**
Install Rust using the official RustUp installer:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

### **3. Reload the shell environment to include Cargo**
```sh
source $HOME/.cargo/env
```

---

### **4. Install dependencies required for SQLx**
Install the necessary packages to compile and link dependencies for SQLx:
```sh
sudo apt install -y build-essential pkg-config libssl-dev libsqlite3-dev pkg-config git
```

---

### **5. Install SQLx CLI with PostgreSQL support**
Install the SQLx CLI tool with PostgreSQL support:
```sh
cargo install sqlx-cli --no-default-features --features postgres
```

---

### **6. Install the dependencies for AWS-LC-RS**
Install the dependencies required to work with AWS-LC-RS (for certificate handling support in Rust’s `rustls`):
```sh
sudo apt install -y cmake ninja-build clang pkg-config
```

---

### **7. Install PostgreSQL and required extensions**
Install PostgreSQL along with useful extensions:
```sh
sudo apt install -y postgresql postgresql-contrib
```

---

### **8. Start and enable the PostgreSQL service**
Start PostgreSQL and set it to run at startup:
```sh
sudo systemctl start postgresql
sudo systemctl enable postgresql
```

---

### **9. Configure the PostgreSQL database**
Switch to the PostgreSQL user:
```sh
sudo -i -u postgres
```

Create a new database:
```sql
CREATE DATABASE mydatabase;
```

Create a new user with encrypted password authentication:
```sql
CREATE USER myuser WITH ENCRYPTED PASSWORD 'mypassword';
```

Grant privileges to the new user:
```sql
GRANT ALL PRIVILEGES ON DATABASE mydatabase TO myuser;
```

Grant permissions on the `public` schema to the user:
```sql
GRANT ALL ON SCHEMA public TO myuser;
ALTER SCHEMA public OWNER TO myuser;
ALTER DATABASE mydatabase OWNER TO myuser;
```

Exit the PostgreSQL shell:
```sql
\q
exit
```

---

### **10. Clone the Axium repository**
Clone the Axium repository from GitHub:
```sh
git clone https://github.com/Riktastic/Axium.git
```

---

### **11. Set up the project**
Navigate to the project directory and copy the example environment file:
```sh
cd Axium && cp .env.example .env
```

Edit the `.env` file to set the correct database URL:
```sh
nano .env  # Update to: DATABASE_URL="postgres://myuser:mypassword@localhost/mydatabase"
```

---

### **12. Run the database migrations**
Run the migrations to create the necessary tables:
```sh
sqlx migrate run
```
- If this fails, reinstall SQLx, check the `DATABASE_URL`, or verify your database permissions.

---

### **13. Build and run the Axium application**
Build and run the Axium application:
```sh
cargo run
```
- This will fail if the required tables have not been added to the database.

---

### **14. Set up Redis**

Redis is used as a caching mechanism in Axium. To set it up:

1. **Install Redis**:
   ```sh
   sudo apt install redis-server
   ```

2. **Start Redis service**:
   ```sh
   sudo systemctl start redis
   sudo systemctl enable redis
   ```

3. **Test Redis**:
   After installation, test if Redis is working properly:
   ```sh
   redis-cli ping
   ```
   If Redis is working correctly, you should get the response `PONG`.

4. **Update the `.env` file**:
   Open your `.env` file and configure the Redis endpoint:
   ```sh
   nano .env
   ```

   Set the following values:
   ```env
   CACHE_ENDPOINT="127.0.0.1"
   CACHE_PORT="6379"
   CACHE_USERNAME=""  # Optional if Redis requires authentication
   CACHE_PASSWORD=""  # Optional if Redis requires authentication
   CACHE_DB="0"
   ```

---

### **15. Set up MinIO**

MinIO is a high-performance, S3-compatible object storage service. Follow these steps to set it up:

1. **Install MinIO**:
   Download and install MinIO by running the following commands:
   ```sh
   wget https://dl.min.io/server/minio/release/linux-amd64/minio
   chmod +x minio
   sudo mv minio /usr/local/bin
   ```

2. **Start MinIO**:
   Run MinIO with the following command:
   ```sh
   minio server /data --console-address ":9001"
   ```

   This will start MinIO with the default configuration, serving the storage endpoint at `http://localhost:9000` and the management console at `http://localhost:9001`.

3. **Access the MinIO Console**:
   Open your browser and navigate to:
   ```sh
   http://localhost:9001
   ```
   Log in with the default access key `minioadmin` and secret key `minioadmin`.

4. **Create a Bucket**:
   In the MinIO console, create a bucket to store profile pictures. For example, create a bucket named `profile-pictures`.

5. **Update the `.env` file**:
   Open the `.env` file and update the MinIO storage settings:
   ```sh
   nano .env
   ```

   Set the following values:
   ```env
   STORAGE_HOST="http://localhost"
   STORAGE_PORT="9000"
   STORAGE_CONSOLE_PORT="9001"
   STORAGE_REGION="us-east-1"
   STORAGE_ACCESS_KEY="minioadmin"
   STORAGE_SECRET_KEY="minioadmin"
   STORAGE_BUCKET_PROFILE_PICTURES="profile-pictures"
   ```

---

### **16. Configure Mail (SMTP)**

Axium can send emails for actions like user registration, password reset, etc. To configure the SMTP settings:

1. **Edit the `.env` file**:
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


### **17. Final Steps**

#### **Run the Axium Application**
For running the application in development, follow these steps.

1. Run Axium by executing the following command in the project folder:
   ```sh
   cargo run
   ```
2. If everything starts up without errors, test your Axium application by visiting: 
   - `http://127.0.0.1:8000`
  
With these instructions, you should now have **Axium**, **Redis**, **MinIO**, and **Mail (SMTP)** properly set up on your **Ubuntu 24.04.01 LTS** server.

#### **Release the Axium Application**
To release your Axium application (i.e., compile it for production or deploy it), follow these steps:

1. **Build the Release Version**
   - For optimized production builds, use the following command:
     ```sh
     cargo build --release
     ```
   - This command will compile the project in release mode, optimizing the binary for performance. The output will be located in the `target/release` directory.

2. **Check the Build Output**
   - After the build completes, navigate to the `target/release` directory:
     ```sh
     cd target/release
     ```
   - You should see an executable file named after your project (e.g., `axium.exe`).

3. **Running the Release Version**
   - You can now run the release version of your application:
     ```sh
     ./axium.exe
     ```
   - This will start the application without the extra debug information, and it will be optimized for production use.