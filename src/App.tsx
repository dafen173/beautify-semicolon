import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";


function App() {
  const [input, setInput] = useState('');
  const [output, setOutput] = useState('');


  const handleCommand = async (command: string) => {
    try {
      const result = await invoke<string>(command, { data: input });
      setOutput(result);
    } catch (error) {
      setOutput("Error: " + error);
    }
  }

  return (
    <div className="app">
      <header className="app-header">
        <h1>Code Beautifier</h1>
      </header>
      <div className="container">
        <div className="box">
          <h2>Input</h2>
          <textarea
            className="textarea"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            placeholder="Enter your code or data here..."
          ></textarea>
        </div>

        <div className="box">
          <h2>Output</h2>
          <textarea
            className="textarea"
            value={output}
            readOnly
            placeholder="Output will be here..."
          ></textarea>
        </div>
      </div>
      <div className="button-container">
        <button onClick={() => handleCommand("beautify")}>
          Beautify JSON
        </button>
        <button onClick={() => handleCommand("minify")}>
          Minify JSON
        </button>
        <button onClick={() => handleCommand("json_to_yaml")}>
          JSON to YAML
        </button>
        <button onClick={() => handleCommand("json_to_xml")}>
          JSON to XML
        </button>
      </div>
    </div>
  );


}


export default App;