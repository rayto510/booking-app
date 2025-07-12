import { useState } from "react";
import { createBooking } from "../lib/api";

export function BookingForm() {
  const [form, setForm] = useState({
    name: "",
    email: "",
    service_type: "",
    date: "",
    time_slot: "",
  });
  const [success, setSuccess] = useState(false);

  const handleChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>
  ) => {
    setForm({ ...form, [e.target.name]: e.target.value });
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const res = await createBooking(form);
    if (res?.id) {
      setSuccess(true);
      setForm({
        name: "",
        email: "",
        service_type: "",
        date: "",
        time_slot: "",
      });
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4 max-w-md mx-auto">
      <input
        name="name"
        placeholder="Name"
        value={form.name}
        onChange={handleChange}
        required
        className="w-full border p-2"
      />
      <input
        name="email"
        placeholder="Email"
        value={form.email}
        onChange={handleChange}
        required
        className="w-full border p-2"
      />
      <input
        name="service_type"
        placeholder="Service Type"
        value={form.service_type}
        onChange={handleChange}
        required
        className="w-full border p-2"
      />
      <input
        type="date"
        placeholder="Date"
        name="date"
        value={form.date}
        onChange={handleChange}
        required
        className="w-full border p-2"
      />
      <input
        name="time_slot"
        placeholder="Time Slot"
        value={form.time_slot}
        onChange={handleChange}
        required
        className="w-full border p-2"
      />
      <button
        type="submit"
        className="bg-blue-600 text-white px-4 py-2 rounded"
      >
        Submit
      </button>
      {success && <p className="text-green-600">Booking created!</p>}
    </form>
  );
}
