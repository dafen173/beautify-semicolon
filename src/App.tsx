import { invoke } from '@tauri-apps/api/core';
import './App.css';

function App() {
  const sendCommand = (cmd: string) => {
    // console.log("Command sent:", cmd);


    invoke('send_projector_command', { cmd })
      .then((response) => console.log("Success:", response))
      .catch((error) => console.error("Error999:", error));

      console.log("Command sent:", cmd);
  };

  return (
    <div className="app">
      <h1>Optoma Projector Control</h1>
      <div className="button-grid">
        <button onClick={() => sendCommand("~0000 1")}>Power On</button>
        <button onClick={() => sendCommand("~0000 0&2")}>Power Off</button>
        <button onClick={() => sendCommand("~00140 20")}>Menu</button>
        <button onClick={() => sendCommand("~0012 1")}>HDMI</button>
        <button onClick={() => sendCommand("~0012 5")}>VGA</button>
        <button onClick={() => sendCommand("~0012 10")}>Video</button>
      </div>
    </div>
  );
}

export default App;





















// import { useState } from "react";
// import { invoke } from "@tauri-apps/api/core";
// import "./App.css";


// function App() {
//   const [input, setInput] = useState('');
//   const [output, setOutput] = useState('');


//   const handleCommand = async (command: string) => {
//     try {
//       const result = await invoke<string>(command, { data: input });
//       setOutput(result);
//     } catch (error) {
//       setOutput("Error: " + error);
//     }
//   }


//   function sendCommand() {
//     invoke('send_projector_command', { cmd: '~00 1 1\r' })  // Power On
//       .then((response) => console.log("Response:", response))
//       .catch((error) => console.error("Error999:", error));
//   }


//   return (
//     <div className="app">
//       <header className="app-header">
//         <h1>Code Beautifier</h1>
//       </header>
//       <div className="container">
//         <div className="box">
//           <h2>Input</h2>
//           <textarea
//             className="textarea"
//             value={input}
//             onChange={(e) => setInput(e.target.value)}
//             placeholder="Enter your code or data here..."
//           ></textarea>
//         </div>

//         <div className="box">
//           <h2>Output</h2>
//           <textarea
//             className="textarea"
//             value={output}
//             readOnly
//             placeholder="Output will be here..."
//           ></textarea>
//         </div>
//       </div>
//       <div className="button-container">
//         <button onClick={() => handleCommand("beautify")}>
//           Beautify JSON
//         </button>
//         <button onClick={() => handleCommand("minify")}>
//           Minify JSON
//         </button>
//         <button onClick={() => handleCommand("json_to_yaml")}>
//           JSON to YAML
//         </button>
//         <button onClick={() => handleCommand("json_to_xml")}>
//           JSON to XML
//         </button>


//         <button onClick={sendCommand}>
//           Power On
//         </button>
//       </div>
//     </div>
//   );

// }


// export default App;
