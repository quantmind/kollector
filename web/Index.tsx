import CssBaseline from "@mui/material/CssBaseline";
import { createTheme, ThemeProvider } from "@mui/material/styles";
import React from "react";
import { createRoot } from "react-dom/client";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import Main from "./Main";

const root = document.getElementById("__kollector");

if (root) {
  const app = createRoot(root);

  const darkTheme = createTheme({
    palette: {
      mode: "dark",
    },
  });

  app.render(
    <ThemeProvider theme={darkTheme}>
      <CssBaseline />
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Main />} />
        </Routes>
      </BrowserRouter>
    </ThemeProvider>
  );
}
