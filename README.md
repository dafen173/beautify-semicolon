# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)




===========================


tauri.conf.json

{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "my-app",
  "version": "0.1.0",
  "identifier": "com.myapp.app",
  "build": {
    "beforeDevCommand": "yarn dev",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "yarn build",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "my-app",
        "width": 800,
        "height": 600
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
