import { useState } from "react";
import femboySocks from "./assets/femboySocks.png";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import "./titlebar/titlebar.css"
import {appWindow} from "@tauri-apps/api/window";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
      <div style={{width: "100%"}}>
          <div data-tauri-drag-region="FUCK OFF INTELLISENSE" className="titlebar">
              <div className="titlebar-button" id="titlebar-close" onClick={() => appWindow.minimize()}>
                  <img src="https://api.iconify.design/mdi:close.svg" alt="close"/>
              </div>
          </div>
          <div style={{marginTop: "35px", width: "100%"}}>
              {/* INSANE FEMBOY MENU HEADER */}
              <div style={{display: "flex", width: "100%"}}>
                  <img src={femboySocks} className="App-logo" alt="logo" style={{
                      width: "150px",
                  }} />
                  <div style={{display: "flex", justifyContent: "center", width: "100%"}}>
                      <h2>FEMBOYWARE</h2>
                  </div>
                  <img src={femboySocks} className="App-logo" alt="logo" style={{
                      width: "150px",
                  }} />
              </div>
              do the gay here
          </div>
      </div>
      
  );
}

export default App;
