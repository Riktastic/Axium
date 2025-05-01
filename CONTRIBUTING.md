## 🤝 Contributing

We welcome contributions to the Axium project! Whether it's fixing bugs, improving documentation, or adding new features, your help is greatly appreciated. Please follow these guidelines to ensure a smooth contribution process.

### 📝 How to Contribute

1. **Fork the Repository**  
   Start by forking the repository to your own GitHub account.

2. **Clone Your Fork**  
   Clone your forked repository to your local machine:
   ```bash
   git clone https://github.com/your-username/Axium.git
   cd Axium
   ```

3. **Create a New Branch**  
   Create a new branch for your feature or bug fix:
   ```bash
   git checkout -b feature-name
   ```

4. **Make Your Changes**  
   Make the necessary changes to the code or documentation. Make sure to write tests for new features and adhere to the existing code style.

5. **Check Dependencies**  
   Before committing your changes, make sure your `Cargo.toml` only includes the necessary dependencies. To do this, run:
   ```bash
   cargo install cargo-udeps
   cargo udeps +nightly
   ```
   This will help ensure there are no unnecessary dependencies in your `Cargo.toml`. If you see any unused dependencies, remove them to keep the project clean and efficient.

6. **Commit Your Changes**  
   Commit your changes with a clear, descriptive message:
   ```bash
   git commit -m "Add feature XYZ or fix issue ABC"
   ```

7. **Push to Your Fork**  
   Push your changes to your fork:
   ```bash
   git push origin feature-name
   ```

8. **Open a Pull Request**  
   Open a pull request against the `main` branch of the original repository. In the description, provide details about the changes you made, the problem they solve, and any testing you performed.

### 🔍 Code Style

- Follow the **Rust style guidelines** outlined in the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/).
- Use **cargo fmt** to automatically format your code:
  ```bash
  cargo fmt
  ```
- Write **meaningful commit messages** that describe the changes you've made.

### 🛠️ Reporting Bugs

If you encounter a bug or issue, please check if it has already been reported in the [GitHub issues](https://github.com/Riktastic/Axium/issues). If not, create a new issue, providing the following information:

- A clear description of the problem.
- Steps to reproduce the issue.
- Expected vs. actual behavior.
- Any relevant logs or error messages.

## 📝 To-Do:
Things to work on.

- Check the register handlers and the insert user handlers for possible security risks,
- Remove almost duplicate user manipulation database functions,
- Check the authorize middleware for possible security risks,
- Simplify the user models.


### 💬 Discussion

Feel free to open discussions in the [Discussions](https://github.com/Riktastic/Axium/discussions) section for general questions, ideas, or advice on how to improve the project.

### 🧑‍💻 Code of Conduct

Please be respectful and follow the [Code of Conduct](https://www.contributor-covenant.org/) while interacting with other contributors. Let's maintain a positive and welcoming environment.

### 🎉 Thanks for Contributing!

Your contributions help make Axium better for everyone! 🙏