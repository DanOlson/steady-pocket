import React from 'react'
import {
  BrowserRouter as Router,
  Link
} from 'react-router-dom'
import Routes from './Routes'
import './App.css'

function App() {
  return (
    <Router>
      <div className="App">
        <header className="App-header">
          <nav className="App-header-menu">
            <ul>
              <li className="App-header-nav-item">
                <Link to="/budgets">Budgets</Link>
              </li>
            </ul>
          </nav>
        </header>
        <div className="content container">
          <Routes />
        </div>
      </div>
    </Router>
  );
}

export default App;
