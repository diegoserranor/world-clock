import { ReactNode } from "react";
import ClockList from "./screens/clock-list";
import ClockAdd from "./screens/clock-add";
import Titlebar from "./lib/titlebar";
import { Navigation } from "./lib/nav";
import "./app.css";

function Layout({ children }: { children: ReactNode }) {
  return (
    <div className="flex flex-col items-center justify-start min-h-screen px-14 py-7 bg-[#000000] text-white">
      <Titlebar />
      {children}
    </div>
  );
}

export default function App() {
  return (
    <Layout>
      <Navigation clockList={<ClockList />} clockAdd={<ClockAdd />} />
    </Layout>
  );
}

