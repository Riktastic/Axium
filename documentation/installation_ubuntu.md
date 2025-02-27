# Setup Instructions for Axium on Ubuntu 24.04.01 LTS

This guide walks you through setting up the **Axium** project on an Ubuntu system. It covers installation of necessary tools like Rust, SQLx, PostgreSQL, and AWS-LC-RS.

Please note! That in a production system I wouldn't want to run the database on the same server as the API.

---

### **1. Update the System**
```sh
sudo apt update && sudo apt upgrade -y
```

---

### **2. Install Rust Using rustup**
Install Rust using the official rustup installer:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

### **3. Reload the Shell Environment to Include Cargo**
```sh
source $HOME/.cargo/env
```

---

### **4. Install Dependencies Required for SQLx**
Install the necessary packages to compile and link dependencies for SQLx:
```sh
sudo apt install -y build-essential pkg-config libssl-dev libsqlite3-dev pkg-config git
```

---

### **5. Install SQLx CLI with PostgreSQL Support**
Install the SQLx CLI tool with PostgreSQL support:
```sh
cargo install sqlx-cli --no-default-features --features postgres
```

---

### **6. Install Dependencies for AWS-LC-RS**
Install the dependencies required to work with AWS-LC-RS, which adds certificate handling support to Rust’s `rustls`:
```sh
sudo apt install -y cmake ninja-build clang pkg-config
```

---

### **7. Install PostgreSQL and Required Extensions**
Install PostgreSQL along with useful extensions:
```sh
sudo apt install -y postgresql postgresql-contrib
```

---

### **8. Start and Enable PostgreSQL Service**
Start PostgreSQL and set it to run at startup:
```sh
sudo systemctl start postgresql
sudo systemctl enable postgresql
```

---

### **9. Configure the PostgreSQL Database**
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

### **10. Clone the Axium Repository**
Clone the Axium repository from GitHub:
```sh
git clone https://github.com/Riktastic/Axium.git
```

---

### **11. Set Up the Project**
Navigate to the project directory and copy the example environment file:
```sh
cd Axium && cp .env.example .env
```

Edit the `.env` file to set the correct database URL:
```sh
nano .env  # Update to: DATABASE_URL="postgres://myuser:mypassword@localhost/mydatabase"
```

---

### **12. Run Database Migrations**
Run the migrations to create the necessary tables:
```sh
sqlx migrate run
```
- If this fails, reinstall SQLx or verify your database permissions.

---

### **13. Build and Run the Axium Application**
Build and run the Axium application:
```sh
cargo run
```
- This will fail if the required tables have not been added to the database.

---