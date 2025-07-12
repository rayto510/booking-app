import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { AdminDashboard } from "./AdminDashboard";
import * as api from "../lib/api";

vi.mock("../lib/api");

describe("AdminDashboard", () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  it("renders bookings and responds to filter changes", async () => {
    vi.spyOn(api, "getBookings").mockResolvedValue([
      {
        id: "1",
        name: "Jane Doe",
        email: "jane@example.com",
        service_type: "Nails",
        date: "2025-07-25",
        time_slot: "14:00-15:00",
      },
    ]);

    render(<AdminDashboard />);

    expect(api.getBookings).toHaveBeenCalled();

    await waitFor(() => {
      expect(screen.getByText(/Jane Doe/)).toBeInTheDocument();
      expect(
        screen.getByText(/Nails on 2025-07-25 at 14:00-15:00/)
      ).toBeInTheDocument();
    });

    fireEvent.change(screen.getByPlaceholderText("Service Type"), {
      target: { value: "Nails" },
    });

    await waitFor(() => {
      expect(api.getBookings).toHaveBeenCalledWith({
        date: "",
        service_type: "Nails",
      });
    });
  });
});
