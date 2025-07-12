import { useEffect, useState } from "react";
import { getBookings } from "../lib/api";

type Booking = {
  id: string;
  name: string;
  email: string;
  service_type: string;
  date: string;
  time_slot: string;
};

export function AdminDashboard() {
  const [bookings, setBookings] = useState<Booking[]>([]);
  const [filters, setFilters] = useState({ date: "", service_type: "" });

  useEffect(() => {
    getBookings(filters).then(setBookings);
  }, [filters]);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setFilters({ ...filters, [e.target.name]: e.target.value });
  };

  return (
    <div className="max-w-4xl mx-auto p-4">
      <h2 className="text-xl font-semibold mb-4">All Bookings</h2>
      <div className="mb-4 flex gap-4">
        <input
          name="date"
          type="date"
          value={filters.date}
          onChange={handleChange}
          className="border p-2"
        />
        <input
          name="service_type"
          placeholder="Service Type"
          value={filters.service_type}
          onChange={handleChange}
          className="border p-2"
        />
      </div>
      <ul className="space-y-2">
        {bookings.map((b) => (
          <li key={b.id} className="border p-4 rounded">
            <div className="font-medium">
              {b.name} ({b.email})
            </div>
            <div>
              {b.service_type} on {b.date} at {b.time_slot}
            </div>
          </li>
        ))}
      </ul>
    </div>
  );
}
