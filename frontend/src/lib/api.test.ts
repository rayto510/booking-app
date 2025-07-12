import { describe, it, expect, vi, beforeEach } from "vitest";
import { getBookings, createBooking } from "./api";

beforeEach(() => {
  vi.restoreAllMocks();
});

describe("api", () => {
  it("fetches bookings with filters", async () => {
    const mockResponse = [{ name: "Alice" }];
    vi.stubGlobal(
      "fetch",
      vi.fn().mockResolvedValue({
        json: () => Promise.resolve(mockResponse),
      })
    );

    const result = await getBookings({ date: "2025-07-20" });

    expect(fetch).toHaveBeenCalledWith(
      "http://localhost:8080/bookings?date=2025-07-20"
    );
    expect(result).toEqual(mockResponse);
  });

  it("creates a booking", async () => {
    const bookingData = {
      name: "Bob",
      email: "bob@example.com",
      service_type: "Haircut",
      date: "2025-07-21",
      time_slot: "10:00-11:00",
    };
    const mockResponse = { id: "123", ...bookingData };

    vi.stubGlobal(
      "fetch",
      vi.fn().mockResolvedValue({
        json: () => Promise.resolve(mockResponse),
      })
    );

    const result = await createBooking(bookingData);

    expect(fetch).toHaveBeenCalledWith(
      "http://localhost:8080/bookings",
      expect.objectContaining({
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(bookingData),
      })
    );
    expect(result).toEqual(mockResponse);
  });
});
