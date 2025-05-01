Sure! Here's an updated version of your instructions with the Redis setup revised to use WSL or Docker, as the official Redis project for Windows is inactive:

---

# **Windows 11 23H2 Setup Instructions for Axium**

This guide walks you through setting up the **Axium** project on a Windows 11 system. It includes installation of necessary tools like Git, Rust, SQLx, PostgreSQL, AWS-LC-RS, MinIO, and Redis.

---

### **1. Install Git for Windows**
1. Download & install the latest version of Git from: [https://git-scm.com/downloads/win](https://git-scm.com/downloads/win) (preferably the standalone 64-bit installer).
2. When prompted with installation options, configure them as follows:
   - Deselect the options you don't need.
   - Use the default installation configuration (everything except desktop shortcuts).
   - For "Choosing the default editor used by Git," select your favorite IDE (e.g., Visual Studio Code).
   - For "Adjusting the name of the initial branch in new repositories," choose the default ("Let Git decide").
   - For "Adjusting your PATH environment," select: *Git from the command line and also from third-party software*.
   - For "Choosing the SSH executable," select: *Use bundled OpenSSH*.
   - For "Choosing the HTTPS transport backend," select: *Use the OpenSSL library*.
   - For "Configuring line-ending conversions," choose: *Checkout Windows-style, commit Unix-style line endings*.
   - For "Configuring the terminal emulator to use with Git Bash," select: *Use Windows' default console window*.
   - For "Choosing the default behavior of 'git pull,'" select: *Fast-forward or merge*.
   - For "Choose a credential helper," select: *Git Credential Manager*.
   - For "Configuring extra options," select: *Enable file system caching*.

---

### **2. Install Rust**
1. Download & install the latest installer from: [https://rustup.rs/](https://rustup.rs/) (preferably x64).
   - Current installation options: 1) Proceed with the standard installation.

---

### **3. Install NASM (required for AWS-LC-RS)**
1. Download & install the latest version of NASM from: [https://nasm.us/](https://nasm.us/).
   - Download the installer for `/win64` and run it as Administrator.
   - Use the default installation settings.

---

### **4. Install PostgreSQL**
1. Download & install the latest version of PostgreSQL from: [https://www.enterprisedb.com/downloads/postgres-postgresql-downloads](https://www.enterprisedb.com/downloads/postgres-postgresql-downloads).
   - Use the default installation options.
   - Make sure to write down the password for the `postgres` user during installation.

4.1. Open PGAdmin and create a new database.

---

### **5. Install the Visual Studio Build Tools**
1. Download & install the Visual Studio 2022 Build Tools from: [https://visualstudio.microsoft.com/downloads/?q=build+tools](https://visualstudio.microsoft.com/downloads/?q=build+tools).
   - Install the Visual Studio Installer.
   - Open the "Individual components" tab and select:
     - C++ CMake tools for Windows
     - Windows 11 SDK
     - MSVC V143 (Build tools)

---

### **6. Install CMake**
1. Download & install the latest version of CMake from: [https://cmake.org/download/](https://cmake.org/download/).
   - Ensure that CMake is added to the PATH environment variable.
   - The Visual Studio 2022 installer may add CMake to the PATH automatically.

---

### **7. Close all terminal sessions**
Before continuing, make sure to close all terminal sessions like CMD.exe, PowerShell.exe, and VS Code to ensure new environment variables take effect.

---

### **8. Clone the Axium repository**
Clone the Axium repository from GitHub:
```sh
git clone https://github.com/Riktastic/Axium.git
```

---

### **9. Install SQLx CLI**
Install the SQLx CLI tool with PostgreSQL support:
```sh
cargo install sqlx-cli --no-default-features --features postgres
```

---

### **10. Set up the project**
Navigate to the project directory and copy the example environment file:
```sh
cd Axium && cp .env.example .env
```

Edit the `.env` file with a text editor to set the correct database URL. For example:
```sh
DATABASE_URL="postgres://postgres:1234@localhost/mydatabase"
```

---

### **11. Run database migrations**
Run the migrations to create the necessary tables:
```sh
sqlx migrate run
```
- If this fails, reinstall SQLx, check the `DATABASE_URL`, or verify your database permissions.

---

### **12. Build and run the Axium application**
Build and run the Axium application:
```sh
cargo run
```
- This will fail if the required tables have not been added to the database.

---

### **13. Install MinIO (for local object storage)**

MinIO is an S3-compatible object storage service. Here’s how to set it up on Windows:

1. **Download MinIO for Windows**: Go to [MinIO's download page](https://min.io/download) and get the Windows binary.
   
2. **Run MinIO**:
   - Open Command Prompt or PowerShell.
   - Navigate to the folder where MinIO was downloaded and run:
     ```sh
     minio.exe server C:\minio\data --console-address ":9001"
     ```
   - This will start MinIO with the web console accessible at `http://127.0.0.1:9001`.

3. **Access the MinIO Web Console**:
   - Open your browser and go to [http://127.0.0.1:9001](http://127.0.0.1:9001).
   - The default credentials are:
     - **Access Key**: `minioadmin`
     - **Secret Key**: `minioadmin`
   
4. **Create a Bucket**:
   - After logging in, create a bucket named `profile-pictures` (or another name as per your `.env` configuration).

---

### **14. Install Redis**

Redis is used for caching. Since the official Redis for Windows project is no longer active, we recommend using Redis within **WSL** (Windows Subsystem for Linux) or **Docker**.

#### **Option 1: Using Redis with WSL**

1. **Install WSL** (if not already installed):
   - Open PowerShell as Administrator and run the following command to install WSL and set the default version to WSL 2:
     ```sh
     wsl --install --set-default-version 2
     ```

2. **Install Redis on WSL**:
   - Open your WSL terminal (you can use Ubuntu or any other Linux distribution).
   - Install Redis by running:
     ```sh
     sudo apt update
     sudo apt install redis-server
     ```

3. **Start Redis**:
   - Start the Redis service:
     ```sh
     sudo service redis-server start
     ```

4. **Verify Redis is running**:
   - To check if Redis is running, you can run:
     ```sh
     redis-cli ping
     ```
     - If Redis is running correctly, you should get the response `PONG`.

5. **Update the `.env` file**:
   - Edit your `.env` file to point to Redis in WSL:
     ```sh
     nano .env
     ```

     Set the following values:
     ```env
     CACHE_ENDPOINT="127.0.0.1"
     CACHE_PORT="6379"
     CACHE_USERNAME=""
     CACHE_PASSWORD=""
     CACHE_DB="0"
     ```

#### **Option 2: Using Redis with Docker**

1. **Install Docker Desktop**:
   - Download Docker Desktop for Windows from [here](https://www.docker.com/products/docker-desktop).
   - Follow the installation instructions and make sure to enable WSL 2 integration during installation.

2. **Run Redis via Docker**:
   - Once Docker is installed and running, open a terminal (PowerShell or CMD) and run the following command to start a Redis container:
     ```sh
     docker run --name redis -p 6379:6379 -d redis
     ```

3. **Verify Redis is running**:
   - To check if the Redis container is running, use:
     ```sh
     docker ps
     ```
   - If Redis is running correctly, it should show up in the list of containers.

4. **Update the `.env` file**:
   - Edit your `.env` file to point to Redis running in Docker:
     ```sh
     nano .env
     ```

     Set the following values:
     ```env
     CACHE_ENDPOINT="127.0.0.1"
     CACHE_PORT="6379"
     CACHE_USERNAME=""
     CACHE_PASSWORD=""
     CACHE_DB="0"
     ```

---

### **15. Configure Mail (SMTP)**

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

### **16. Final steps**
Run Axium by executing: "cargo run" in the project folder using CMD or PowerShell.
If everything starts up nicely, test your Axium application by visiting: `http://127.0.0.1:8000`.

Sure! Below is an updated version of the **Final Steps** section, with additional instructions for releasing the application:

---

### **16. Final Steps**

#### **Run the Axium Application**
For running the application in development, follow these steps.

1. Run Axium by executing the following command in the project folder using CMD or PowerShell:
   ```sh
   cargo run
   ```
2. If everything starts up without errors, test your Axium application by visiting: 
   - `http://127.0.0.1:8000`
  
Now you should have **Axium** running with **PostgreSQL**, **MinIO**, and **Redis** (via WSL or Docker) on your Windows 11 machine!

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