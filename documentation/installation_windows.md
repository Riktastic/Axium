# Windows 11 23H2 Setup Instructions for Axium

This guide walks you through setting up the **Axium** project on a Windows 11 system. It includes installation of necessary tools like Git, Rust, SQLx, PostgreSQL, and AWS-LC-RS.

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
- If this fails, reinstall SQLx, check the `DATABASE_URL` or verify your database permissions.

---

### **12. Build and run the Axium application**
Build and run the Axium application:
```sh
cargo run
```
- This will fail if the required tables have not been added to the database.

---