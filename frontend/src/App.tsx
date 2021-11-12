import React, { useEffect, useState } from "react";
import logo from "./logo.svg";
import "./App.css";
import axios from "axios";

interface User {
  id: number;
  name: string;
  email: string;
}

function App() {
  const [user, setUser] = useState<User>();

  useEffect(() => {
    axios
      .get("/api")
      .then(({ data }) => {
        setUser(data);
      })
      .catch((e) => {
        console.error(e);
      });
  }, []);

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>Hello {user?.name} !!</p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
