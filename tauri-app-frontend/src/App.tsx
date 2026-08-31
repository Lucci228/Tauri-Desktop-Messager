import { useState } from "react";
import PingAlarm from "./components/PingAlarm"
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import NotificationBar from "./components/NotificationBar";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="container">
      <div className="main-container">
        <NotificationBar></NotificationBar>
        <PingAlarm />
      </div>
    </main>
  );
}

export default App;
