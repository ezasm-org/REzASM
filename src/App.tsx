import { BrowserRouter, Routes, Route, useNavigate } from "react-router-dom";
import Header from './components/Header';
import Button from './components/Button';
import Code from './components/Code';

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
                            <Button />
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
