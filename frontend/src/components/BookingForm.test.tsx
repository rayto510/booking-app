import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { BookingForm } from "./BookingForm";
import * as api from "../lib/api";

vi.mock("../lib/api");

describe("BookingForm", () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  it("submits the form and shows success message", async () => {
    vi.spyOn(api, "createBooking").mockResolvedValue({ id: "123" });

    render(<BookingForm />);

    fireEvent.change(screen.getByPlaceholderText("Name"), {
      target: { value: "Alice" },
    });
    fireEvent.change(screen.getByPlaceholderText("Email"), {
      target: { value: "alice@example.com" },
    });
    fireEvent.change(screen.getByPlaceholderText("Service Type"), {
      target: { value: "Haircut" },
    });
    fireEvent.change(screen.getByPlaceholderText("Time Slot"), {
      target: { value: "10:00-11:00" },
    });
    fireEvent.change(screen.getByPlaceholderText(/date/i), {
      target: { value: "2025-07-21" },
    });

    fireEvent.click(screen.getByRole("button", { name: /submit/i }));

    await waitFor(() => {
      expect(screen.getByText("Booking created!")).toBeInTheDocument();
    });

    expect(api.createBooking).toHaveBeenCalledWith({
      name: "Alice",
      email: "alice@example.com",
      service_type: "Haircut",
      date: "2025-07-21",
      time_slot: "10:00-11:00",
    });
  });
});
