# Ryzen PowerHouse

# NOT FOR GENERAL USE YET!!

**Ryzen PowerHouse** is a Tauri-based application that provides a user-friendly graphical interface for adjusting AMD power management settings using the popular `ryzenadj` tool. The app lets you tweak key parameters of your Ryzen system, save your settings as presets, and quickly apply them via the system tray.

## Features

-   **GUI for AMD Power Management**  
    Adjust the most critical parameters:

    -   **Sustained Power Limit** (`--stapm-limit`)
    -   **Actual/Fast Power Limit** (`--fast-limit`)
    -   **Average/Slow Power Limit** (`--slow-limit`)
    -   **Tctl Temperature Limit** (`--tctl-temp`)
    -   **VRM Current Limits** (`--vrm-current` and `--vrmmax-current`)
    -   **Prochot Deassertion Ramp** (`--prochot-deassertion-ramp`)
    -   Flag options for enabling **Power Saving** and **Maximum Performance**

-   **User-Defined Presets**

    -   Save your preferred configurations as presets
    -   Update the system tray dynamically with your custom presets.
    -   Quickly apply a preset through the tray context menu.

-   **Tray Integration & Background Operation**
    -   The application runs in the background with a system tray icon.
    -   Right-click the tray to access a submenu of your saved presets, open the configuration window, or exit the app.
    -   Closing the main window minimizes the app rather than quitting.

## Contributing

Contributions are welcome! Please open issues or submit pull requests on GitHub with your improvements or bug fixes.

## License

This project is licensed under the GNU General Public License version 3 (GPLv3). See the [LICENSE](LICENSE) file for details.
