// src/lib/api.ts
export async function getBookings(filters?: {
  date?: string;
  service_type?: string;
}) {
  const query = new URLSearchParams(filters).toString();
  const res = await fetch(`http://localhost:8080/bookings?${query}`);
  return res.json();
}

export async function createBooking(data: {
  name: string;
  email: string;
  service_type: string;
  date: string;
  time_slot: string;
}) {
  const res = await fetch("http://localhost:8080/bookings", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(data),
  });
  return res.json();
}
