# Tauri Counter App with Franken UI

This is a simple Tauri application that demonstrates a counter with state management and UI updates using Franken UI. The app allows you to increment, decrement, and reset a counter. When the counter reaches zero, a mock external API call is triggered, and when it reaches five, a different trigger is activated.

## Prerequisites

Before you begin, ensure you have the following installed:

- [Node.js](https://nodejs.org/) (v16 or higher)
- [Rust](https://www.rust-lang.org/tools/install)
- [Tauri CLI](https://tauri.app/start/prerequisites/)

## Cloning the Repository

To clone the repository, run the following command:

```bash
git clone https://github.com/garthtrickett/tauri-demo.git
cd tauri-demo
````

## Installing Dependencies

### Frontend Dependencies

The frontend dependencies are managed by Node.js. To install them, run:

```bash
npm install 
```

### Backend Dependencies

The backend dependencies are managed by Cargo (Rust's package manager). They will be installed automatically when you build the project.

## Running the Application

To run the application, use the following command:

```bash
npm run tauri dev
```

This will start the Tauri development server, compile the Rust backend, and launch the application.

## Project Structure

- `src-tauri/src/main.rs`: Contains the Rust backend logic, including state management and event handling.
- `src/main.ts`: Contains the frontend logic, including UI rendering and event listeners.
- `index.html`: The main HTML file that includes the necessary styles and scripts for the application.

## Key Features

- **State Management**: The application state is managed in Rust and shared with the frontend via Tauri commands and events.
- **Mock API Call**: When the counter reaches zero, a mock API call is triggered, simulating an external API call.
- **UI Updates**: The UI is updated in real-time using `lit-html` and Franken UI components.

## Customization

You can customize the application by modifying the Rust backend or the TypeScript frontend. The `SAMManager` struct in `main.rs` handles the state and actions, while the `renderApp` function in `main.ts` handles the UI rendering.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Tauri](https://tauri.app/) for providing a lightweight and secure framework for building desktop applications.
- [Franken UI](https://github.com/franken-ui/franken-ui) for providing a modern and customizable UI component library.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have any improvements or bug fixes.

## Support

If you encounter any issues or have any questions, feel free to open an issue on the GitHub repository.

---

Enjoy building with Tauri and Franken UI! 🚀
