//
//  launch.js
//  matter
// 
//  Created by Wess Cope (me@wess.io) on 03/24/20
//  Copyright 2019 Wess Cope
//

const path  = require("path");
const isDev = require("electron-is-dev");

const {
  app, 
  BrowserWindow
} = require("electron")

let mainWindow

function createWindow() {
  mainWindow = new BrowserWindow({
    width: 900,
    height: 680
  });

  mainWindow.loadURL(
    isDev
      ? "http://localhost:1234"
      : `file://${path.join(__dirname, "../dist/index.html")}`
  );
  mainWindow.on("closed", () => (mainWindow = null));
}

app.on("ready", createWindow);

app.on("window-all-closed", () => {
  if (process.platform !== "darwin") {
    app.quit();
  }
});

app.on("activate", () => {
  if (mainWindow === null) {
    createWindow();
  }
});
