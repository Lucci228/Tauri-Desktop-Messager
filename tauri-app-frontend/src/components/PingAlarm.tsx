import { useState } from "react";
import { ping_server, PingResponse } from "../api/ping_api";
import "./PingAlarm.css"

function PingAlarm() {
  const [message, setMsg] = useState("Test");
  const handlePing = async () => {
    console.log("Ping");
    let response : PingResponse = await ping_server();
    setMsg(`Ping ${response.status} count ${response.ping_count}`);
  }

  return (
    <div className="alarm-content">
      <div>
        {message}
      </div>
      <div>
        <button onClick={handlePing} className="ping-button"> Ping </button>
      </div>
    </div>
  )
}


export default PingAlarm;
