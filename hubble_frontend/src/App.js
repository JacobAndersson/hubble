import Analysis from "./components/Analysis";
import Chooser from "./components/Chooser";
import { Routes, Route } from "react-router-dom";

import "./App.css";

function App() {
  return (
    <Routes>
      <Route path="/" element={<Chooser />} />
      <Route path="/analyse/:id" element={<Analysis />} />
    </Routes>
  );
}

export default App;
