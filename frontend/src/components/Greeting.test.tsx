// src/components/Greeting.test.tsx
import { render, screen } from "@testing-library/react";
import { Greeting } from "./Greeting";

test("renders a greeting", () => {
  render(<Greeting name="Raymond" />);
  expect(screen.getByText("Hello, Raymond!")).toBeInTheDocument();
});
