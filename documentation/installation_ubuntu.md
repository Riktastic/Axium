# Setup Instructions for Axium on Ubuntu 24.04.01 LTS

This guide walks you through setting up the **Axium** project on an Ubuntu system. It covers installation of necessary tools like Rust, SQLx, PostgreSQL, and AWS-LC-RS.

Please note! That in a production system I wouldn't want to run the database on the same server as the API.

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
Install the dependencies required to work with AWS-LC-RS, which adds certificate handling support to Rust’s `rustls`:
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
- If this fails, reinstall SQLx, check the `DATABASE_URL` or verify your database permissions.

---

### **13. Build and run the Axium application**
Build and run the Axium application:
```sh
cargo run
```
- This will fail if the required tables have not been added to the database.

---