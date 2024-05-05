import React, { useState } from 'react';
import { BrowserRouter as Router, Switch, Route, Link, Redirect } from 'react-router-dom';

const env = {
    REACT_APP_API_URL: 'http://yourapiurl.com'
};

const Login = ({onLogin}) => {
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");

    const handleSubmit = (e) => {
        e.preventDefault();
        console.log(`Login with ${email} and ${password}`);
        onLogin(true); 
    };

    return (
        <div>
            <h2>Login</h2>
            <form onSubmit={handleSubmit}>
                <div>
                    <label>Email:</label>
                    <input type="email" value={email} onChange={(e) => setEmail(e.target.value)} required />
                </div>
                <div>
                    <label>Password:</label>
                    <input type="password" value={password} onChange={(e) => setPassword(e.target.value)} required />
                </div>
                <button type="submit">Login</button>
            </form>
        </div>
    );
};

const Register = () => {
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");

    const handleSubmit = (e) => {
        e.preventDefault();
        console.log(`Register with ${email} and ${password}`);
    };

    return (
        <div>
            <h2>Register</h2>
            <form onSubmit={handleSubmit}>
                <div>
                    <label>Email:</label>
                    <input type="email" value={email} onChange={(e) => setEmail(e.target.value)} required />
                </div>
                <div>
                    <label>Password:</label>
                    <input type="password" value={password} onChange={(e) => setPassword(e.target.value)} required />
                </div>
                <button type="submit">Register</button>
            </form>
        </div>
    );
};

const Projects = () => {
    return (
        <div>
            <h2>Projects</h2>
        </div>
    );
};

const TimeLog = () => {
    return (
        <div>
            <h2>Time Log</h2>
        </div>
    );
};

const Summary = () => {
    return (
        <div>
            <h2>Summary</h2>
        </div>
    );
};

const App = () => {
    const [isLoggedIn, setIsLoggedIn] = useState(false);

    return (
        <Router>
            <div>
                <nav>
                    <ul>
                        <li><Link to="/register">Register</Link></li>
                        <li><Link to="/login">Login</Link></li>
                        {isLoggedIn && (
                            <>
                                <li><Link to="/projects">Projects</Link></li>
                                <li><Link to="/timelog">Time Log</Link></li>
                                <li><Link to="/summary">Summary</Link></li>
                            </>
                        )}
                    </ul>
                </nav>
                <Switch>
                    <Route path="/register">
                        <Register />
                    </Route>
                    <Route path="/login">
                        <Login onLogin={setIsLoggedIn} />
                    </Route>
                    <Route path="/projects">
                        {isLoggedIn ? <Projects /> : <Redirect to="/login" />}
                    </Route>
                    <Route path="/timelog">
                        {isLoggedIn ? <TimeLog /> : <Redirect to="/login" />}
                    </Route>
                    <Route path="/summary">
                        {isLoggedIn ? <Summary /> : <Redirect to="/login" />}
                    </Route>
                    <Route path="/">
                        <Redirect to="/login" />
                    </Route>
                </Switch>
            </div>
        </Router>
    );
};

export default App;