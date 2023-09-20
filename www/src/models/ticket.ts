
interface TicketModel {
  id: string;
  number: number;
  name: string;
  email: string;
  message: string;
  note: string|null;
  status: "open"|"pending"|"closed";
  created_at: string;
  updated_at: string|null;
  closed_at: string|null;
}

export default TicketModel;