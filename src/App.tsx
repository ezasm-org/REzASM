import { BrowserRouter, Routes, Route, useNavigate } from "react-router-dom";
import Header from './components/Header';
import Button from './components/Button';

import About from './components/About';
import Features from "./components/Features";
import Code from './components/Code';
import Contributors from "./components/Contributors";

const HOME_PATH = "/";
const CODE_PATH = "/code";
const DOWNLOAD_PATH = "/downloads";

const App = () => {
    return (
        <BrowserRouter>
            <Routes>
                <Route 
                    path={HOME_PATH} 
                    element={
                        <div className="pt-[4.75rem] lg:pt-[5.25rem] overflow-hidden">
                            <Header />
                            <About />
                            <Features />
                            <Button />
                            <Contributors />
                        </div>
                    } 
                />
                <Route path={CODE_PATH} element={<Code />} />
            </Routes>
        </BrowserRouter>
    );
};

export default App;

export { HOME_PATH, CODE_PATH, DOWNLOAD_PATH };
