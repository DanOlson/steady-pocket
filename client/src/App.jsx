import React from 'react'
import {
  BrowserRouter as Router,
  Link
} from 'react-router-dom'
import Routes from './Routes'
import ThemeToggle from './components/ThemeToggle'
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
          <ThemeToggle />
        </header>
        <div className="content">
          <Routes />
        </div>
      </div>
    </Router>
  );
}

export default App;
